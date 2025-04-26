use crate::{Board, game::utils::Player};

use super::action::Action;

pub(crate) struct BitBoard {
    current: u64,
    other: u64,
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

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> Vec<Action> {
        let mut pcs = (!Self::TOP) & self.current & !Self::LEFT;
        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src) << Self::TOP_LEFT_MV;
            let mut capture = false;

            let self_on_target = self.current & tgt != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !Self::TOP & !Self::LEFT) != 0)
                && ((tgt >> Self::TOP_LEFT_MV) & (self.current | self.other) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                tgt = tgt << Self::TOP_LEFT_MV;
                capture = true;
            }

            let tgt = tgt.trailing_zeros() as u8;
            mvs.push(Action { src, tgt, capture });
        }

        mvs
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<Action> {
        let mut pcs = (!BitBoard::TOP) & self.current & (!BitBoard::RIGHT);
        let mut mvs = Vec::with_capacity(pcs.count_ones() as usize);

        while pcs != 0 {
            let src = pcs.trailing_zeros() as u8;
            pcs &= pcs - 1;

            let mut tgt = (1 << src) << Self::TOP_RIGHT_MV;
            let mut capture = false;

            let self_on_target = self.current & tgt != 0;
            let enemy_on_target = self.other & tgt != 0;
            let valid_capture = ((tgt & !Self::TOP & !Self::RIGHT) != 0)
                && ((tgt >> Self::TOP_RIGHT_MV) & (self.current | self.other) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                tgt = tgt << Self::TOP_RIGHT_MV;
                capture = true;
            }

            let tgt = tgt.trailing_zeros() as u8;
            mvs.push(Action { src, tgt, capture });
        }

        mvs
    }

    fn bottom_right(&self) -> Vec<Action> {
        let mut src = (!BitBoard::BOTTOM) & self.current & (!BitBoard::RIGHT);
        let mut mvs = Vec::with_capacity(src.count_ones() as usize);

        while src != 0 {
            let from = src.trailing_zeros() as u8;
            src &= src - 1;

            let mut to = (1 << from) >> Self::BOTTOM_RIGHT_MV;
            let mut capture = false;

            let self_on_target = self.current & to != 0;
            let enemy_on_target = self.other & to != 0;
            let valid_capture = ((to & !Self::BOTTOM & !Self::RIGHT) != 0)
                && ((to >> Self::BOTTOM_RIGHT_MV) & (self.current | self.other) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                to = to >> Self::BOTTOM_RIGHT_MV;
                capture = true;
            }

            let tgt = to.trailing_zeros() as u8;
            let src = from;
            mvs.push(Action { src, tgt, capture })
        }

        mvs
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

    pub(super) fn new(current: u64, other: u64) -> Self {
        Self { current, other }
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    pub(crate) fn bottom_left(&self) -> Vec<Action> {
        let mut src = ((!BitBoard::LEFT) & self.current) & (!BitBoard::BOTTOM);
        let mut moves = Vec::with_capacity(src.count_ones() as usize); // (from, to)

        while src != 0 {
            let from = src.trailing_zeros() as u8;
            src &= src - 1;

            let mut to = (1 << from) >> Self::BOTTOM_LEFT_MV;
            let mut capture = false;

            let self_on_target = self.current & to != 0;
            let enemy_on_target = self.other & to != 0;
            let valid_capture = ((to & !Self::LEFT & !Self::BOTTOM) != 0)
                && ((to >> Self::BOTTOM_LEFT_MV) & (self.current | self.other) == 0);

            if self_on_target || (enemy_on_target && !valid_capture) {
                continue;
            }

            if enemy_on_target {
                to = to >> Self::BOTTOM_LEFT_MV;
                capture = true;
            }

            moves.push(Action {
                src: from,
                tgt: to.trailing_zeros() as u8,
                capture,
            });
        }

        moves
    }
}

impl From<(u64, u64)> for BitBoard {
    fn from(value: (u64, u64)) -> Self {
        Self {
            current: value.0,
            other: value.1,
        }
    }
}
