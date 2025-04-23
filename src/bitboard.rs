use std::ops::{Deref, DerefMut};

use crate::direction::Dir;

pub(crate) struct BitBoard(u64);

impl BitBoard {
    const LEFT: u64 = 0x101010101010101;
    const RIGHT: u64 = 0x8080808080808080;
    const BOTTOM: u64 = 0xff;
    const TOP: u64 = 0xff00000000000000;

    const TOP_LEFT_MV: u8 = 7;
    const TOP_RIGHT_MV: u8 = 9;
    const BOTTOM_LEFT_MV: u8 = 9;
    const BOTTOM_RIGHT_MV: u8 = 7;

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 8 (top row)
    /// pieces that are safe to move top-left
    fn top_left(&self) -> Vec<(u8, u8)> {
        let src = ((!BitBoard::LEFT) & self.0) & ((!BitBoard::TOP) & self.0);
        let dst = src << Self::TOP_LEFT_MV;

        return Self::get_moves(src, dst);
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<(u8, u8)> {
        let src = ((!BitBoard::RIGHT) & self.0) & ((!BitBoard::TOP) & self.0);
        let dst = src << Self::TOP_RIGHT_MV;

        return Self::get_moves(src, dst);
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    fn bottom_left(&self) -> Vec<(u8, u8)> {
        let src = ((!BitBoard::LEFT) & self.0) & ((!BitBoard::BOTTOM) & self.0);
        let dst = src >> Self::BOTTOM_LEFT_MV;

        return Self::get_moves(src, dst);
    }

    fn bottom_right(&self) -> Vec<(u8, u8)> {
        let src = ((!BitBoard::RIGHT) & self.0) & ((!BitBoard::BOTTOM) & self.0);
        let dst = src >> Self::BOTTOM_RIGHT_MV;

        return Self::get_moves(src, dst);
    }

    /// returns Vec<(from, to)>
    fn get_moves(mut src: u64, mut dst: u64) -> Vec<(u8, u8)> {
        let mut moves = Vec::with_capacity(src.count_ones() as usize); // (from, to)

        while src != 0 {
            let from = src.trailing_zeros() as u8;
            let to = dst.trailing_zeros() as u8;

            moves.push((from, to));

            src &= src - 1;
            dst &= dst - 1;
        }

        moves
    }

    pub(crate) fn moves(&self, direction: Dir) -> Vec<(u8, u8)> {
        let (mut left, mut right) = match direction {
            Dir::Top => (self.top_left(), self.top_right()),
            Dir::Bottom => (self.bottom_left(), self.bottom_right()),
        };

        left.reserve(right.len());
        left.append(&mut right);

        left
    }
}

impl Deref for BitBoard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
