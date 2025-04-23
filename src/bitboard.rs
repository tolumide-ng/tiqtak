use crate::utils::Player;

pub(crate) struct BitBoard {
    pub(super) current: u64,
    pub(super) other: u64,
}

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
    fn top_left(&self) -> Vec<(u8, u8, bool)> {
        let src = ((!BitBoard::LEFT) & self.current) & ((!BitBoard::TOP) & self.current);
        let dst = src << Self::TOP_LEFT_MV;

        return self.get_moves(src, dst);
    }

    /// exclude the pieces already on column H (right column)
    /// exclude the pieces already on row 8 (top row)
    fn top_right(&self) -> Vec<(u8, u8, bool)> {
        let src = ((!BitBoard::RIGHT) & self.current) & ((!BitBoard::TOP) & self.current);
        let dst = src << Self::TOP_RIGHT_MV;

        return self.get_moves(src, dst);
    }

    /// exclude the pieces already on column A (left column)
    /// exclude the pieces already on row 1 (bottom row)
    fn bottom_left(&self) -> Vec<(u8, u8, bool)> {
        let src = ((!BitBoard::LEFT) & self.current) & ((!BitBoard::BOTTOM) & self.current);
        let dst = src >> Self::BOTTOM_LEFT_MV;

        return self.get_moves(src, dst);
    }

    fn bottom_right(&self) -> Vec<(u8, u8, bool)> {
        let src = ((!BitBoard::RIGHT) & self.current) & ((!BitBoard::BOTTOM) & self.current);
        let dst = src >> Self::BOTTOM_RIGHT_MV;

        return self.get_moves(src, dst);
    }

    /// returns Vec<(from, to, capture)>
    fn get_moves(&self, mut src: u64, mut dst: u64) -> Vec<(u8, u8, bool)> {
        let mut moves = Vec::with_capacity(src.count_ones() as usize); // (from, to)

        while src != 0 {
            let from = src.trailing_zeros() as u8;
            let to = dst.trailing_zeros() as u8;

            let capture = ((1 << to) & self.other) != 0;

            moves.push((from, to, capture));

            src &= src - 1;
            dst &= dst - 1;
        }

        moves
    }

    pub(crate) fn moves(&self, direction: Player) -> Vec<(u8, u8, bool)> {
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
}

impl From<(u64, u64)> for BitBoard {
    fn from(value: (u64, u64)) -> Self {
        Self {
            current: value.0,
            other: value.1,
        }
    }
}
