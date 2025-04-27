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

    fn move_north(&self, mask: u64, shift: u8) -> Vec<Action> {
        let mut pcs = (!Self::TOP) & self.current & !mask;
        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src) << shift;
            let mut capture = false;

            let self_on_target = self.current & tgt != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !Self::TOP & !mask) != 0)
                && ((tgt << shift) & (self.current | self.other | self.team) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                let new_others = (self.other & !tgt);
                tgt = tgt << shift;
                let new_current = (self.current & !(1 << src)) | tgt | self.team;
                capture = true;

                let board = Board::with(new_current, new_others, 0, Player::South);
                let mut result = board
                    .options(Player::South)
                    .into_iter()
                    .filter(|x| x.capture && tgt.trailing_zeros() as u8 == x.tgt)
                    .collect::<Vec<_>>();

                mvs.reserve(result.len());
                mvs.append(&mut result);
            }

            let tgt = tgt.trailing_zeros() as u8;
            mvs.push(Action { src, tgt, capture });
        }

        mvs
    }

    fn move_south(&self, mask: u64, shift: u8) -> Vec<Action> {
        let mut pcs = (!Self::BOTTOM) & self.current & (!mask);
        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src) >> shift;
            let mut capture = false;

            let self_on_target = self.current & tgt != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !Self::BOTTOM & !mask) != 0)
                && ((tgt >> shift) & (self.current | self.other | self.team) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                let new_others = (self.other & !tgt);
                tgt = tgt >> shift;
                let new_current = (self.current & !(1 << src)) | tgt | self.team;
                capture = true;

                // Assumption here: we don't care whether this is a king or nah, we also don't care
                let board = Board::with(new_current, new_others, 0, Player::North);
                let mut result = board
                    .options(Player::North)
                    .into_iter()
                    .filter(|x| x.capture && tgt.trailing_zeros() as u8 == x.tgt)
                    .collect::<Vec<_>>();

                mvs.reserve(result.len());
                mvs.append(&mut result);
            }

            let tgt = tgt.trailing_zeros() as u8;
            mvs.push(Action { src, tgt, capture })
        }

        mvs
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> Vec<Action> {
        self.move_north(Self::LEFT, Self::TOP_LEFT_MV)
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<Action> {
        self.move_north(Self::RIGHT, Self::TOP_RIGHT_MV)
    }

    fn bottom_right(&self) -> Vec<Action> {
        self.move_south(Self::RIGHT, Self::BOTTOM_RIGHT_MV)
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    pub(crate) fn bottom_left(&self) -> Vec<Action> {
        self.move_south(Self::LEFT, Self::BOTTOM_LEFT_MV)
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

        println!("{board}");

        let mvs = board.options(Player::South);
        assert_eq!(mvs.len(), 6);
    }

    #[test]
    fn should_return_all_south_moves_including_kings() {
        let north = 0x520000a00000000u64;
        let south = 0x40014200000u64;

        let kings = 1 << 42;

        let board = Board::with(north, south, kings, Player::South);
        let received = board.options(Player::South);

        let expected = [
            (26u8, 40u8, true),
            (26, 44, true),
            (28, 37, false),
            (21, 30, false),
            (42, 24, true),
            (42, 49, false),
            (42, 51, false),
        ];

        assert_eq!(received.len(), expected.len());

        expected
            .iter()
            .for_each(|mv| assert!(received.contains(&Action::from(*mv))));
    }

    // should return all mulitple moves (for a single piece) in one go for a regular player test (bottom-left -->> bottom-right)
    // same as above, but testing for kings
    // #[test]
    // fn should_return_all_possible_moves_in_the_start_position() {
    //     let south = 0x200008000001u64;
    //     let north = 0x40000000000000u64;

    //     let kings = 1 << 42;

    //     let board = Board::with(north, south, kings, Player::North);
    //     let received = board.options(Player::North);

    //     let expected = [(56, 49, false), (58, 49, false), (58, 51, false)];

    //     println!("the board here is ****** \n {board}");
    //     assert!(false);
    // }

    // should_return_all_possible_moves_in_the_start_position
}
