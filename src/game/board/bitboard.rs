use crate::game::model::bits::Bits;
use crate::game::model::player::Player;
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

    fn next<F>(&self, src: u8, tgt: u8, turn: Player, captured: u8, mut func: F)
    where
        F: FnMut(ActionPath),
    {
        if tgt >= 32 {
            return;
        }
        let kings = self.current & self.kings;
        let promoted = ((1u32 << tgt) & turn.opponent_base()) != 0;

        let capture = captured != 0;
        let parent = Action::new(src, tgt, capture, promoted, U32);

        let current = 1 << tgt;
        let others = self.other & !(1 << captured);
        let team = (self.team & !(1 << src)) | (self.current & !(1 << src)) | current;
        // let mut kings =
        //     (kings & !(1 << src)) | (current * (u32::from((kings & 1 << src != 0) || promoted)));

        // if moving piece is a king, remove from previous position
        // if the captured piece is a king, remove it
        // if this piece was just promoted, or if it is a king that just moved, register it at the target position
        let is_king = (kings & 1 << src) != 0;
        let kings =
            ((kings & !(1 << src)) & !(1 << captured)) | (u32::from(is_king || promoted) << tgt);

        // // we need to remove the captured if they were a king
        // if capture {
        //     kings = kings & !(1 << captured);
        // }

        if capture {
            let result = BitBoard::new(current, others, team, kings).get(turn);
            result.into_iter().for_each(|mut actions| {
                if let Some(act) = actions.peek(actions.len() - 1) {
                    if act.capture {
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

        let empty = !(self.current | self.other | self.team);
        let kings = self.current & self.kings;

        // regular jumps
        let temp = empty.shift_by(4, turn) & self.other; // any enemy on the left? (current's right)
        let jump = (((temp & turn.s3()).shift_by(3, turn))
            | ((temp & turn.s5()).shift_by(5, turn)))
            & self.current;

        for src in Bits::new(jump) {
            // let left = ((turn.s3().shift_by(src, !turn)) & 1) != 0;
            // let dir = if left { 3 } else { 5 };
            let dir = 5;
            let shift = 4;
            let tgt = src.move_by(dir + shift, turn);
            let captured = src.move_by(dir, turn);

            self.next(src, tgt, turn, captured, |path| {
                mvs.push(path);
            });
        }

        let temp = (((empty & turn.s3()).shift_by(3, turn))
            | ((empty & turn.s5()).shift_by(5, turn)))
            & self.other;

        for src in Bits::new((temp.shift_by(4, turn)) & self.current) {
            let left = (turn.s3().shift_by(src.move_by(4, turn), !turn)) & 1 != 0;
            let dir = if left { 5 } else { 3 };
            let shift = 4;
            let tgt = src.move_by(dir + shift, turn);
            let captured = src.move_by(shift, turn);

            if src == 22 {
                println!("!!!!!!!!!!!!!ABBBBBBB src={src} tgt==>>{tgt} {shift} ndir-> {dir}");
                let mut b = Board::new();
                // b.south = self.current;
                // b.north = self.other;
                // b.kings = kings;
                b.south = self.current;
                b.north = self.other;
                b.kings = kings;
                println!("{}", b);
            }

            self.next(src, tgt, turn, captured, |path| {
                mvs.push(path);
            });
        }

        if kings != 0 {
            // king jumpers (jumping in both directions i.e North, and South)
            let tempk0 = (empty.shift_by(4, !turn)) & self.other;

            let jumpk0 = (((tempk0 & (!turn).s3()).shift_by(3, !turn))
                | ((tempk0 & (!turn).s5()).shift_by(5, !turn)))
                & kings;

            let tempk1 = (((empty & (!turn).s3()).shift_by(3, !turn))
                | ((empty & (!turn).s5()).shift_by(5, !turn)))
                & self.other;
            let jumpk1 = (tempk1.shift_by(4, !turn)) & kings;

            let jumping_kings = [(jumpk0, 2, 3), (jumpk1, 4, 4)];

            for (jumper, shift, victim) in jumping_kings {
                for src in Bits::new(jumper) {
                    let right = ((!turn).s3().shift_by(src.move_by(shift, !turn), !turn)) & 1 != 0;
                    let dir = if right { 3 } else { 5 };
                    let tgt = src.move_by(dir + shift, !turn);

                    let captured = src.move_by(victim, !turn);

                    if src == 29 {
                        println!("src -->> {src}, tgt -->> {tgt}, captured==>>{captured}");
                    }
                    self.next(src, tgt, turn, captured, |path| {
                        mvs.push(path);
                    });
                }
            }
        }

        // (movers, target, is_king)
        let movers = [
            ((empty.shift_by(4, turn)), 4u8, false), // regular piece
            (((empty & turn.s3()).shift_by(3, turn)), 3, false), // regular piece
            (((empty & turn.s5()).shift_by(5, turn)), 5, false), // regular piece
            ((empty.shift_by(4, !turn)), 4u8, true), // king move
            (((empty & (!turn).s3()).shift_by(3, !turn)), 3, true), // king move
            (((empty & (!turn).s5()).shift_by(5, !turn)), 3, true), // king move
        ];

        for (bits, shift, is_king) in movers {
            let pieces = if is_king { kings } else { self.current };
            let dir = if is_king { !turn } else { turn };
            for src in Bits::new(bits & pieces) {
                let tgt = src.move_by(shift, dir);

                self.next(src, tgt, turn, 0, |path| {
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

    use crate::{
        Scale::{self, *},
        game::{board::state::Board, utils::Qmvs},
    };

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

        println!("{board}");

        received.iter().for_each(|p| {
            p.mvs[..p.len]
                .iter()
                .for_each(|a| print!("{} ---->>>", Action::from(*a)));
            println!("\n\n\n");
        });

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
        // let north = 0x244u64;
        // let south = 0xaa00000000000000u64;

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
        // println!("{board}");

        assert_eq!(board.kings.count_ones(), 6);
        assert_eq!((board.south & board.kings).count_ones(), 4);
        assert_eq!(board.north.count_ones(), 3);
        assert_eq!((board.north & (!board.kings)).count_ones(), 1);
        assert_eq!((board.north & board.kings).count_ones(), 2);

        assert!((board.kings & (1 << 1)) != 0);
        assert!((board.kings & (1 << 5)) == 0);

        let action = ActionPath::from(Action::from((2, 11, false, false, U64)));

        println!("{board}");

        let new_board = board.play(action).unwrap();

        assert_eq!(new_board.kings.count_ones(), 6);
        assert_eq!((new_board.south & board.kings).count_ones(), 4);
        assert_eq!(new_board.north.count_ones(), 3);
        assert!((new_board.kings & (1 << 1)) == 0);
        assert!((new_board.kings & (1 << 5)) != 0);
        assert_eq!((new_board.north & (!new_board.kings)).count_ones(), 1);
        assert_eq!((new_board.north & new_board.kings).count_ones(), 2);
    }

    // #[test]
    // fn should_be_able_to_play_a_jump_game() {}

    // #[test]
    // fn should_make_only_valid_moves() {
    //     let xxx = 0x11200000;
    //     let b = Board::with(xxx, 0, 0, Player::North, Qmvs::default());
    //     println!("{}", b);

    //     // let north = 0x11200000;
    //     // let south = 0x26000;
    //     let north = 1 << 22;
    //     let south = 1 << 19;

    //     let board = Board::with(north, south, 0, Player::North, Qmvs::default());
    //     println!("{board}");

    //     assert!(false);

    //     // let received = board.options(Player::North);

    //     // let expected = get_path(vec![
    //     //     vec![(54u8, 47u8, false, false, true)],
    //     //     vec![(45u8, 38u8, false, false, true)],
    //     // ]);

    //     // println!(
    //     //     ":first :::: : {:?}",
    //     //     getax(Action::from((54u8, 47u8, false, false)))
    //     // );

    //     // expected.iter().for_each(|x| {
    //     //     let rr = received.iter().for_each(|a| {
    //     //         let abx = Action::from(a[0]).transcode();
    //     //         println!("the x here is {:?} {:?}", abx, abx.to_string());
    //     //     });
    //     //     assert!(received.contains(&x))
    //     // });

    //     // // expected.iter().for_each(|x| assert!(received.contains(&x)));
    //     // assert_eq!(received.len(), expected.len());
    // }
}
