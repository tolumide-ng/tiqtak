use crate::game::u64_shift::U64Ext;
use crate::{Board, game::utils::Player};

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
    fn get(&self, hor_mask: u64, shift: u8, turn: Player) -> Vec<Action> {
        // vertical mask
        let v_mask = match turn {
            Player::South => Self::TOP,
            Player::North => Self::BOTTOM,
        };

        // South
        let mut pcs = (!v_mask) & self.current & !hor_mask;
        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src).shift_by(shift, turn);
            let mut capture = false;

            let self_on_target = self.current & tgt != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !v_mask & !hor_mask) != 0)
                && ((tgt.shift_by(shift, turn)) & (self.current | self.other | self.team) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                let new_others = self.other & !tgt;
                // println!("original tgt is************** {}", tgt.trailing_zeros());
                tgt = tgt.shift_by(shift, turn);
                // println!("THE NEW TARGET IS ++++++++++++++  {}", tgt.trailing_zeros());

                let new_current = (self.current & !(1 << src)) | tgt | self.team;
                capture = true;

                // if this current pirce is a king, we need to be able to continue with that process here

                let board = match turn {
                    Player::North => Board::with(new_current, new_others, 0, turn),
                    Player::South => Board::with(new_others, new_current, 0, turn),
                };

                // println!("the board after {board}");
                let result = board.options(turn);

                // println!("the target here is>>>>>>>> {}", tgt.trailing_zeros());
                // println!("did we get anyyyy [[[[[[---]]]]]]] \n {:?} \n", result);

                let mut result = result
                    .into_iter()
                    .filter(
                        |x| x.capture, // && tgt.trailing_zeros() as u8 == x.tgt
                    )
                    .collect::<Vec<_>>();

                // println!("did we get anyyyy result??????????? \n {:?}", result);

                mvs.reserve(result.len());
                mvs.append(&mut result);
            }

            let tgt = tgt.trailing_zeros() as u8;

            let promoted = match turn {
                Player::South => (tgt / 8) == 7,
                Player::North => (tgt / 8) == 0,
            };

            mvs.push(Action {
                src,
                tgt,
                capture,
                promoted,
            });
        }

        mvs
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> Vec<Action> {
        self.get(Self::LEFT, Self::TOP_LEFT_MV, Player::South)
        // vec![]
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<Action> {
        self.get(Self::RIGHT, Self::TOP_RIGHT_MV, Player::South)
        // vec![]
    }

    fn bottom_right(&self) -> Vec<Action> {
        self.get(Self::RIGHT, Self::BOTTOM_RIGHT_MV, Player::North)
        // vec![]
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    pub(crate) fn bottom_left(&self) -> Vec<Action> {
        self.get(Self::LEFT, Self::BOTTOM_LEFT_MV, Player::North)
        // vec![]
    }

    pub(crate) fn moves(&self, direction: Player) -> Vec<Action> {
        let (mut left, mut right) = match direction {
            Player::South => (self.top_left(), self.top_right()),
            Player::North => (self.bottom_left(), self.bottom_right()),
        };

        left.reserve(right.len());
        left.append(&mut right);

        left
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
    use super::*;

    #[test]
    fn should_return_all_possible_moves_for_south_player() {
        let north = 0x520000a00000000u64;
        let south = 0x40014200000u64;

        let board = Board::with(north, south, 0, Player::South);
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

        let board = Board::with(north, south, kings, Player::South);
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

        let board = Board::with(north, south, kings, Player::North);
        let received = board.options(Player::North);

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
        let received = board.options(Player::North);
        assert_eq!(received.len(), 7);
        assert_eq!(board.options(Player::South).len(), 7);
    }
    // should convert a regular to a king after they reach the opponents base
}
