use std::collections::HashSet;

use crate::Board;
use crate::game::u64_shift::U64Ext;
use crate::game::utils::Player;

use super::action::Action;

pub(crate) struct BitBoard {
    current: u64,
    other: u64,
    team: u64,
}

impl BitBoard {
    const LEFT: u64 = 0x101010101010101;
    const RIGHT: u64 = 0x8080808080808080;
    pub(crate) const BOTTOM: u64 = 0xff;
    pub(crate) const TOP: u64 = 0xff00000000000000;

    const TOP_LEFT_MV: u8 = 7;
    const TOP_RIGHT_MV: u8 = 9;
    const BOTTOM_LEFT_MV: u8 = 9;
    const BOTTOM_RIGHT_MV: u8 = 7;

    // hor_mask: horizontal mask
    fn get(&self, hor_mask: u64, shift: u8, turn: Player) -> HashSet<Action> {
        // vertical mask
        let v_mask = match turn {
            Player::South => Self::TOP,
            Player::North => Self::BOTTOM,
        };

        // South
        let mut pcs = (!v_mask) & self.current & (!hor_mask);
        // println!(
        //     "the count here is >>>>>>>>>>>>>> {:?} ---------------- {:?}",
        //     pcs.count_ones(),
        //     pcs.trailing_zeros()
        // );

        let mut mvs = HashSet::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src).shift_by(shift, turn);
            let mut capture = false;
            let mut promoted = false;

            let self_on_target = ((self.current | self.team) & tgt) != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !v_mask & !hor_mask) != 0)
                && ((tgt.shift_by(shift, turn)) & (self.current | self.other | self.team) == 0); // ensures landing(jumping target) space is empty

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target && valid_capture {
                let new_others = self.other & !tgt;
                tgt = tgt.shift_by(shift, turn);

                promoted = (tgt.trailing_zeros() / 8) == ((turn as u32) * 7);

                let new_team = (self.current & !(1 << src)) | (self.team & !(1 << src)) | tgt;
                // let new_current = (self.current & !(1 << src)) | self.team;
                capture = true;

                let kings = (promoted as u64) * tgt;

                let board = BitBoard::new(tgt, new_others, new_team);

                let mut result = board.moves(turn);
                (kings != 0).then(|| {
                    let more = board.moves(!turn);
                    result = &result | &more;
                });
                let result = result.into_iter().filter(|x| x.capture).collect();
                //
                mvs = &mvs | &result // combines both (hashset automatically helps us remove duplicates)
            }

            let tgt = tgt.trailing_zeros() as u8;

            promoted = (tgt / 8) == ((turn as u8) * 7);
            mvs.insert(Action {
                src,
                tgt,
                capture,
                promoted,
            });
        }

        mvs
    }

    pub(crate) fn moves(&self, play_as: Player) -> HashSet<Action> {
        let (mut left, right) = match play_as {
            Player::South => (self.top_left(), self.top_right()),
            Player::North => (self.bottom_left(), self.bottom_right()),
        };

        left = &left | &right;

        left
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> HashSet<Action> {
        self.get(Self::LEFT, Self::TOP_LEFT_MV, Player::South)
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> HashSet<Action> {
        self.get(Self::RIGHT, Self::TOP_RIGHT_MV, Player::South)
    }

    fn bottom_right(&self) -> HashSet<Action> {
        self.get(Self::RIGHT, Self::BOTTOM_RIGHT_MV, Player::North)
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    pub(crate) fn bottom_left(&self) -> HashSet<Action> {
        self.get(Self::LEFT, Self::BOTTOM_LEFT_MV, Player::North)
    }

    pub(super) fn new(current: u64, other: u64, team: u64) -> Self {
        Self {
            current,
            other,
            team,
        }
    }
}

impl From<(u64, u64, u64)> for BitBoard {
    fn from(value: (u64, u64, u64)) -> Self {
        Self {
            current: value.0,
            other: value.1,
            team: value.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    use super::*;

    #[test]
    fn should_return_all_possible_moves_for_south_player() {
        let north = 0x520000a00000000u64;
        let south = 0x40014200000u64;

        let board = Board::with(north, south, 0, Player::South, (0, 0));
        let received = board.options(Player::South);

        let expected = [
            (21u8, 30u8, false, false),
            (28u8, 37u8, false, false),
            (26u8, 40u8, true, false),
            (26u8, 44u8, true, false),
            (44u8, 62u8, true, true),
            (42u8, 49u8, false, false),
            (42u8, 51u8, false, false),
        ];

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|x| assert!(received.contains(&Action::from(*x))));
    }

    #[test]
    fn should_return_all_south_moves_including_kings() {
        let north = 0x520000a00000000u64;
        let south = 0x40014200000u64;

        let kings = 1 << 42;

        let board = Board::with(north, south, kings, Player::South, (0, 0));
        let received = board.options(Player::South);

        let expected = [
            (26u8, 40u8, true, false),
            (26, 44, true, false),
            (28, 37, false, false),
            (21, 30, false, false),
            (42, 24, true, false),
            (42, 49, false, false),
            (42, 51, false, false),
            (44, 62, true, true),
        ];

        assert_eq!(received.len(), expected.len());

        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&Action::from(*mv))));
    }

    // should return all mulitple moves (for a single piece) in one go for a regular player test (bottom-left -->> bottom-right)
    // same as above, but testing for kings
    #[test]
    fn should_return_all_multiples_moves_by_one_piece() {
        let south = 0x200008000801u64;
        let north = 0x40000000000000u64;

        let kings = 1 << 42;

        let board = Board::with(north, south, kings, Player::North, (0, 0));
        println!("{board}");
        let received = board.options(Player::North);

        // received.sort();

        received.iter().for_each(|x| println!("{}", x.to_string()));

        let expected = [
            (54u8, 36u8, true, false),
            (54u8, 47u8, false, false),
            (36, 18, true, false),
            (18, 4, true, true),
        ];

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&Action::from(*mv))));
    }

    // should_return_all_possible_moves_in_the_start_position
    #[test]
    fn should_return_all_possible_moves_in_the_base_position() {
        let board = Board::new();
        let received = board.options(Player::South);
        assert_eq!(received.len(), 7);
        assert_eq!(board.options(Player::South).len(), 7);

        let expected = [
            (16, 25, false, false),
            (18, 25, false, false),
            (18, 27, false, false),
            (20, 27, false, false),
            (20, 29, false, false),
            (22, 29, false, false),
            (22, 31, false, false),
        ];

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&Action::from(*mv))));
    }

    // should convert a regular to a king after they reach the opponents base
    #[test]
    fn should_convert_a_regular_to_king_if_they_touch_the_opponents_base() {
        let south = 0x20000000000u64;
        let north = 0x14000008000000u64;

        let board = Board::with(north, south, 0, Player::South, (0, 0));
        println!("{board}");
        let received = board.options(Player::South);

        received.iter().for_each(|x| println!("{}", x.to_string()));

        let expected = [
            (41u8, 59u8, true, true),
            (41, 48, false, false),
            (59, 45, true, false),
        ];

        assert_eq!(received.len(), expected.len());
        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&Action::from(*mv))));

        // assert!(false)
    }

    #[test]
    fn should_make_only_valid_moves() {
        let north = 0x8040200000000000u64;
        let south = 0x1028000000u64;

        let board = Board::with(north, south, 0, Player::North, (0, 0));
        println!("{board}");

        let received = board.options(Player::North);

        let expected = [(54u8, 47u8, false, false), (45u8, 38u8, false, false)];

        expected
            .iter()
            .for_each(|x| assert!(received.contains(&Action::from(*x))));

        assert_eq!(received.len(), expected.len());
    }

    #[test]
    fn a_king_should_never_overwrite_its_teammates() {
        let north = 0x244u64;
        let south = 0xaa00000000000000u64;

        let kings = 1 << 2 | 1 << 6 | 1 << 57 | 1 << 59 | 1 << 61 | 1 << 63;

        let board = Board::with(north, south, kings, Player::North, (0, 0));
        let received = board.options(Player::North);

        let expected = [
            (6, 15, false, false),
            (6, 13, false, false),
            (2, 11, false, false),
            (9, 0, false, true),
        ];

        // received.iter().for_each(|x| {
        //     println!("{:?}", x);
        //     println!("the value is ----->>>>> {} \n", x.to_string())
        // });

        expected
            .iter()
            .for_each(|x| assert!(received.contains(&Action::from(*x))));

        assert_eq!(expected.len(), received.len());
    }

    #[test]
    fn king_piece_should_never_be_demoted_when_it_leaves_the_opponents_base() {
        let north = 0x244u64;
        let south = 0xaa00000000000000u64;

        let kings = 1 << 2 | 1 << 6 | 1 << 57 | 1 << 59 | 1 << 61 | 1 << 63;

        let board = Board::with(north, south, kings, Player::North, (0, 0));
        println!("{board}");

        assert_eq!(board.kings.count_ones(), 6);
        assert_eq!((board.south & board.kings).count_ones(), 4);
        assert_eq!(board.north.count_ones(), 3);
        assert_eq!((board.north & (!board.kings)).count_ones(), 1);
        assert_eq!((board.north & board.kings).count_ones(), 2);
        assert!((board.kings & (1 << 2)) != 0);
        assert!((board.kings & (1 << 11)) == 0);

        let action = Action::from((2, 11, false, false));

        let new_board = board.play(action).unwrap();
        println!("{new_board}");

        assert_eq!(new_board.kings.count_ones(), 6);
        assert_eq!((new_board.south & board.kings).count_ones(), 4);
        assert_eq!(new_board.north.count_ones(), 3);
        assert!((new_board.kings & (1 << 2)) == 0);
        assert!((new_board.kings & (1 << 11)) != 0);
        assert_eq!((new_board.north & (!new_board.kings)).count_ones(), 1);
        assert_eq!((new_board.north & new_board.kings).count_ones(), 2);
    }
}
