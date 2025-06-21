use crate::game::model::bits::Bits;
use crate::game::model::player::Player;
use crate::game::model::sq::Sq;
use crate::game::model::{action::Action, path::ActionPath};
use crate::game::traits::u8_ops::U8Ext;
use crate::game::traits::u32_shift::U32Ext;
use crate::{Board, Qmvs, Scale::*};
// use crate::{Board, Qmvs};

pub(crate) struct BitBoard {
    pub(super) current: u32,
    pub(super) team: u32,
    pub(super) other: u32,
    pub(super) kings: u32,
}

impl BitBoard {
    const LEFT: u32 = 0x08080808;
    const RIGHT: u32 = 0x10101010;
    pub(crate) const BOTTOM: u32 = 0x0000000F;
    pub(crate) const TOP: u32 = 0xF0000000;

    const TOP_LEFT_MV: u8 = 4;
    const TOP_RIGHT_MV: u8 = 5;
    const BOTTOM_LEFT_MV: u8 = 5;
    const BOTTOM_RIGHT_MV: u8 = 4;

    /// Number of rows on a checkers board (for each side)
    const NUM_ROWS: u32 = 4;
    /// On a 0 indexed board (first row as 0), the last row on the board is 7
    /// Yes, there are 8 rows, but its 0 indexed
    const MAX_ROW: u32 = 7;

    const L3_MASK: u32 = 0xE0E0E0E;
    const L5_MASK: u32 = 0x707070;
    const R3_MASK: u32 = 0x70707070;
    const R5_MASK: u32 = 0xE0E0E00;

    //

    const NORTH_LEFT_3: u32 = 0xE0E0E0E; // +3
    const NORTH_LEFT_4: u32 = 0xF0F0F0; // + 4

    const NORTH_RIGHT_4: u32 = 0xF0F0F0F; // + 4
    const NORTH_RIGHT_5: u32 = 0x707070; // + 5

    const SOUTH_LEFT_4: u32 = 0xF0F0F0F0; // + 4
    const SOUTH_LEFT_5: u32 = 0xE0E0E00; // + 5

    const SOUTH_RIGHT_4: u32 = 0xF0F0F0F; // + 4
    const SOUTH_RIGHT_3: u32 = 0x70707070; // + 3

    // /// When calling this function, always ensure that only one bit (i.e the main moving piece) is set in self.current
    // /// All other pieces can be part of `team` or `others`
    /// returns: (src, tgt, capture)
    /// if capture is 0, it means there is no piece for this move to capture
    fn shift(&self, src: u8, turn: Player) -> Vec<(Action, u8)> {
        let curr = 1 << src;
        let empty = !(curr | self.current | self.other | self.team);

        let mut actions: Vec<(Action, u8)> = vec![];

        let mut try_move = |(maska, shfta): (u32, i8), (maskb, shftb): (u32, i8)| {
            let offset =
                (shfta * ((curr & maska) != 0) as i8) + (shftb * ((curr & maskb) != 0) as i8);

            let Ok(mid) = Sq::try_from((src, offset)).map(u8::from) else {
                return;
            };
            let mid_bit = 1 << mid;
            let promoted = (mid_bit & turn.opponent_base()) != 0;

            if (mid_bit & empty) != 0 {
                actions.push((Action::new_32(src, mid, false, promoted), 0));
            } else if (mid_bit & self.other) != 0 {
                let second_offset = (shfta * ((mid_bit & maska) != 0) as i8)
                    + (shftb * ((mid_bit & maskb) != 0) as i8);

                let Ok(tgt) = Sq::try_from((mid, second_offset)).map(u8::from) else {
                    return;
                };
                let tgt_bit = 1 << tgt;
                let promoted = (tgt_bit & turn.opponent_base()) != 0;
                if (tgt_bit & empty) != 0 {
                    actions.push((Action::new_32(src, tgt, true, promoted), mid));
                }
            }
        };
        match turn {
            Player::North => {
                // explores moves towards the south (as a nothern player)
                try_move((Self::SOUTH_LEFT_4, -4), (Self::SOUTH_LEFT_5, -5)); // south-west
                try_move((Self::SOUTH_RIGHT_3, -3), (Self::SOUTH_RIGHT_4, -4)); // south-east
            }
            Player::South => {
                // explores moves towards the north (as a southern player)
                try_move((Self::NORTH_RIGHT_4, 4), (Self::NORTH_RIGHT_5, 5)); // north east
                try_move((Self::NORTH_LEFT_4, 4), (Self::NORTH_LEFT_3, 3)); // north east
            }
        }

        actions
    }

    fn next<F>(&self, action: Action, captured: u8, turn: Player, mut func: F)
    where
        F: FnMut(ActionPath),
    {
        let Action {
            src, tgt, promoted, ..
        } = action;
        if tgt >= 32 {
            return;
        }

        let capture = captured != 0;
        let parent = Action::new(src, tgt, capture, promoted, U32);

        let current = 1 << tgt;
        let others = self.other & !(1 << captured);
        let team = (self.team & !(1 << src)) | (self.current & !(1 << src)) | current;

        // if moving piece is a king, remove from previous position
        // if the captured piece is a king, remove it
        // if this piece was just promoted, or if it is a king that just moved, register it at the target position
        let is_king = (self.kings & 1 << src) != 0;

        let kings = ((self.kings & !(1 << src)) & !(1 << captured))
            | (u32::from(is_king || promoted) << tgt);

        if capture {
            let result = BitBoard::new(current, others, team, kings).get(turn);

            result.into_iter().for_each(|mut actions| {
                if let Some(act) = actions.peek(actions.len() - 1) {
                    let is_cycle = act.tgt == parent.src;
                    if act.capture && !is_cycle {
                        actions.prepend(parent).unwrap();
                        func(actions);
                    }
                }
            });
        }

        func(parent.into());
    }

    pub(crate) fn get(&self, turn: Player) -> Vec<ActionPath> {
        let mut mvs = vec![];

        // let empty = !(self.current | self.other | self.team);
        let kings = self.current & self.kings;

        for src in Bits::from(self.current) {
            let is_king = ((1 << src) & kings) != 0;

            let mut actions = self.shift(src, turn);

            if is_king {
                actions.extend_from_slice(&self.shift(src, !turn));
            }

            for (action, captured) in actions {
                self.next(action, captured, turn, |path| {
                    mvs.push(path);
                });
            }
        }

        mvs
    }

    pub(super) fn new(current: u32, other: u32, team: u32, kings: u32) -> Self {
        Self {
            current,
            other,
            team,
            kings,
        }
    }
}

impl From<(u32, u32, u32, u32)> for BitBoard {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self {
            current: value.0,
            other: value.1,
            team: value.2,
            kings: value.3,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::game::{board::state::Board, utils::Qmvs};

    use super::*;

    fn get_path<T>(input: Vec<Vec<T>>) -> Vec<ActionPath>
    where
        Action: From<T>,
        T: Copy,
    {
        input
            .into_iter()
            .map(|a| {
                ActionPath::try_from(
                    a.iter()
                        .map(|ac| Action::from(*ac).into())
                        .collect::<Vec<u16>>()
                        .as_slice(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn southern_players_jumping_towards_the_south() {
        let south = 1 << 8 | 1 << 10 | 1 << 11;
        let north = 1 << 4 | 1 << 5 | 1 << 6;
        let kings = south;

        let expected = get_path(vec![
            vec![(8, 1, true, true, U32)],
            vec![(10, 1, true, true, U32)],
            vec![(10, 3, true, true, U32)],
            vec![(11, 2, true, true, U32)],
            vec![(11, 2, true, true, U32), (2, 9, true, false, U32)],
            vec![
                (11, 2, true, true, U32),
                (2, 9, true, false, U32),
                (9, 0, true, true, U32),
            ],
            vec![(8, 12, false, false, U32)],
            vec![(10, 13, false, false, U32)],
            vec![(10, 14, false, false, U32)],
            vec![(11, 14, false, false, U32)],
            vec![(11, 15, false, false, U32)],
            vec![(11, 7, false, false, U32)],
        ]);

        let board = Board::with(north, south, kings, Player::South, Qmvs::default());
        let received = board.options(Player::South);

        assert_eq!(expected.len(), received.len());
        expected.iter().for_each(|x| assert!(received.contains(&x)));
    }

    #[test]
    fn northern_players_jumping_towards_the_south() {
        let north = 1 << 12;
        let south = 1 << 9 | 1 << 10 | 1 << 11;
        let kings = 1 << 12 | 1 << 13 | 1 << 14;

        let expected = get_path(vec![
            vec![(12, 5, true, false, U32)],
            vec![(12, 5, true, false, U32), (5, 14, true, false, U32)],
            vec![
                (12, 5, true, false, U32),
                (5, 14, true, false, U32),
                (14, 7, true, false, U32),
            ],
            vec![(12, 16, false, false, U32)],
            vec![(12, 8, false, false, U32)],
            vec![(12, 17, false, false, U32)],
        ]);

        let board = Board::with(north, south, kings, Player::North, Qmvs::default());
        let received = board.options(Player::North);

        assert_eq!(expected.len(), received.len());
        expected.iter().for_each(|x| {
            return assert!(received.contains(&x));
        });
    }

    // should return all mulitple moves (for a single piece) in one go for a regular player test (bottom-left -->> bottom-right)
    // same as above, but testing for kings
    #[test]
    fn should_return_all_multiples_moves_by_one_piece() {
        let south = 1 | 1 << 5 | 1 << 13 | 1u32 << 22;
        let north = 1u32 << 27;
        let kings = 0;

        let board = Board::with(north, south, kings, Player::North, Qmvs::default());
        let received = board.options(Player::North);

        let expected = get_path(vec![
            vec![
                (54u8, 36u8, true, false, U64),
                (36, 18, true, false, U64),
                (18, 4, true, true, U64),
            ],
            vec![(54u8, 36u8, true, false, U64), (36, 18, true, false, U64)],
            vec![(54u8, 36u8, true, false, U64)],
            vec![(54u8, 47u8, false, false, U64)],
        ]);

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|path| assert!(received.contains(&path.transcode())));
    }

    #[test]
    fn should_return_all_possible_moves_for_south_player() {
        let south = 1 << 12 | 1 << 13 | 1 << 20 | 1 << 10;
        let north = 1 << 17 | 1 << 18 | 1 << 27 | 1 << 29;

        let board = Board::with(north, south, 0, Player::South, Qmvs::default());
        let received = board.options(Player::South);

        let expected = get_path(vec![
            vec![(25u8, 43u8, true, false, U64)],
            vec![(25u8, 32u8, false, false, U64)],
            vec![
                (27u8, 45u8, true, false, U64),
                (45u8, 63u8, true, true, U64),
            ],
            vec![((27u8, 45u8, true, false, U64))],
            vec![(20u8, 29u8, false, false, U64)],
            vec![(41u8, 48u8, false, false, U64)],
            vec![(41u8, 50u8, false, false, U64)],
        ]);

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|x| assert!(received.contains(&x.transcode())));
    }

    #[test]
    fn should_return_all_south_moves_including_kings() {
        let north = 1 << 28 | 1 << 29 | 1 << 27 | 1 << 17 | 1 << 18;
        let south = 1 << 11 | 1 << 13 | 1 << 14 | 1 << 21;

        let kings = 1 << 21;

        let board = Board::with(north, south, kings, Player::South, Qmvs::default());
        let received = board.options(Player::South);

        let expected = get_path(vec![
            vec![(27, 45, true, false, U64)],
            vec![(27, 45, true, false, U64), (44, 63, true, true, U64)],
            vec![(27u8, 41u8, true, false, U64)],
            vec![(29, 38, false, false, U64)],
            vec![(22, 31, false, false, U64)],
            vec![(43, 25, true, false, U64)],
            vec![(43, 50, false, false, U64)],
            vec![(43, 52, false, false, U64)],
        ]);

        assert_eq!(received.len(), expected.len());

        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&mv.transcode())));
    }

    #[test]
    fn should_return_all_possible_moves_in_the_base_position() {
        let board = Board::new();
        let received = board.options(Player::South);
        assert_eq!(received.len(), 7);
        assert_eq!(board.options(Player::South).len(), 7);

        let expected = get_path(vec![
            vec![(16, 25, false, false, U64)],
            vec![(18, 25, false, false, U64)],
            vec![(18, 27, false, false, U64)],
            vec![(20, 27, false, false, U64)],
            vec![(20, 29, false, false, U64)],
            vec![(22, 29, false, false, U64)],
            vec![(22, 31, false, false, U64)],
        ]);

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&mv.transcode())));
    }

    // should convert a regular to a king after they reach the opponents base
    #[test]
    fn should_convert_a_regular_to_king_if_they_touch_the_opponents_base() {
        let south = 1 << 20;
        let north = 1 << 25 | 1 << 26 | 1 << 13;

        let board = Board::with(north, south, 0, Player::South, Qmvs::default());
        // println!("{board}");
        let received = board.options(Player::South);
        let expected = get_path(vec![
            vec![(41, 59, true, true, U64), (59, 45, true, false, U64)],
            vec![(41, 59, true, true, U64)],
            vec![(41, 48, false, false, U64)],
        ]);

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&mv.transcode())));
    }

    #[test]
    fn a_king_should_never_overwrite_its_teammates() {
        let north = 1 << 1 | 1 << 3 | 1 << 4;
        let south = 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;
        let kings = 1 << 1 | 1 << 3 | 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;

        let board = Board::with(north, south, kings, Player::North, Qmvs::default());
        let received = board.options(Player::North);

        let expected = get_path(vec![
            vec![(6, 15, false, false, U64)],
            vec![(6, 13, false, false, U64)],
            vec![(2, 11, false, false, U64)],
            vec![(9, 0, false, true, U64)],
        ]);

        assert_eq!(expected.len(), received.len());

        expected
            .iter()
            .for_each(|x| assert!(received.contains(&x.transcode())));
    }

    #[test]
    fn king_piece_should_never_be_demoted_when_it_leaves_the_opponents_base() {
        let north = 1 << 1 | 1 << 3 | 1 << 4;
        let south = 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;

        let kings = 1 << 1 | 1 << 3 | 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;

        let board = Board::with(north, south, kings, Player::North, Qmvs::default());

        assert_eq!(board.kings.count_ones(), 6);
        assert_eq!((board.south & board.kings).count_ones(), 4);
        assert_eq!(board.north.count_ones(), 3);
        assert_eq!((board.north & (!board.kings)).count_ones(), 1);
        assert_eq!((board.north & board.kings).count_ones(), 2);

        assert!((board.kings & (1 << 1)) != 0);
        assert!((board.kings & (1 << 5)) == 0);

        let action = ActionPath::from(Action::from((2, 11, false, false, U64)));

        let new_board = board.play(action).unwrap();

        assert_eq!(new_board.kings.count_ones(), 6);
        assert_eq!((new_board.south & board.kings).count_ones(), 4);
        assert_eq!(new_board.north.count_ones(), 3);
        assert!((new_board.kings & (1 << 1)) == 0);
        assert!((new_board.kings & (1 << 5)) != 0);
        assert_eq!((new_board.north & (!new_board.kings)).count_ones(), 1);
        assert_eq!((new_board.north & new_board.kings).count_ones(), 2);
    }

    #[test]
    fn should_make_only_valid_moves() {
        let north = 1 << 22;
        let south = 1 << 19;

        let board = Board::with(north, south, 0, Player::North, Qmvs::default());
        let received = board.options(Player::North);

        let expected = get_path(vec![
            vec![(45, 31, true, false, U64)],
            vec![(45, 36, false, false, U64)],
        ]);

        assert_eq!(expected.len(), received.len());
        expected.iter().for_each(|x| {
            x.iter().for_each(|x| print!("{}", Action::from(*x)));
            assert!(received.contains(&x.transcode()))
        });
    }

    #[test]
    fn capturing_a_king_removes_such_king() {
        let north = 1 << 22;
        let south = 1 << 19;

        let board = Board::with(north, south, south, Player::North, Qmvs::default());
        let received = board.options(Player::North);
        assert_eq!(524288, board.kings);

        let expected = get_path(vec![
            vec![(45, 31, true, false, U64)],
            vec![(45, 36, false, false, U64)],
        ]);

        println!("{board}");

        assert_eq!(expected.len(), received.len());
        expected.iter().for_each(|x| {
            x.iter().for_each(|x| print!("{}", Action::from(*x)));
            assert!(received.contains(&x.transcode()))
        });

        let action = ActionPath::from(expected[0]);

        let new_board = board.play(action).unwrap();
        println!("{new_board}");

        assert_eq!(new_board.kings, 0);
    }
}
