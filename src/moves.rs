use crate::bitboard::BitBoard;

pub(crate) struct Moves(u64);

impl Moves {
    const TOP_LEFT: u8 = 7;
    const TOP_RIGHT: u8 = 9;
    const BOTTOM_LEFT: u8 = 9;
    const BOTTOM_RIGHT: u8 = 7;

    pub(crate) fn new(pieces: u64) -> Self {
        Self(pieces)
    }

    pub(crate) fn top_moves(&self) {
        // let safe_left = (!BitBoard::LEFT) & self.0;
        // let safe_top = (!BitBoard::TOP) & safe_left;

        // exclude the pieces already on column A (left column)
        // exclude the pieces already on row 8 (top row)
        // pieces that are safe to move top-left
    }

    pub(crate) fn top_left(&self) -> Vec<(u8, u8)> {
        // exclude the pieces already on column A (left column)
        // exclude the pieces already on row 8 (top row)
        // pieces that are safe to move top-left
        let mut source = ((!BitBoard::LEFT) & self.0) & ((!BitBoard::TOP) & self.0);
        let mut destination = source << Self::TOP_LEFT;

        let mut moves = Vec::with_capacity(source.count_ones() as usize); // (from, to)

        while source != 0 {
            let from = source.trailing_zeros() as u8;
            let to = destination.trailing_zeros() as u8;
            moves.push((from, to));

            source &= source - 1;
            destination &= destination - 1;
        }

        moves
    }

    pub(crate) fn top_right(&self) -> Vec<(u8, u8)> {
        // exclude the pieces already on column H (right column)
        //exclude the pieces already on row 8 (top row)

        let mut source = ((!BitBoard::RIGHT) & self.0) & ((!BitBoard::TOP) & self.0);
        let mut destination = source << Self::TOP_RIGHT;

        let mut moves = Vec::with_capacity(source.count_ones() as usize); // (from, to)

        while source != 0 {
            let from = source.trailing_zeros() as u8;
            let to = destination.trailing_zeros() as u8;
            moves.push((from, to));

            source &= source - 1;
            destination &= destination - 1;
        }

        moves
    }

    pub(crate) fn bottom_left(&self) -> Vec<(u8, u8)> {
        // exclude the pieces already on column A (left column)
        // exclude the pieces already on row 1 (bottom row)

        let mut source = ((!BitBoard::LEFT) & self.0) & ((!BitBoard::BOTTOM) & self.0);
        let mut destination = source >> Self::BOTTOM_LEFT;

        let mut moves = Vec::with_capacity(source.count_ones() as usize); // (from, to)

        while source != 0 {
            let from = source.trailing_zeros() as u8;
            let to = destination.trailing_zeros() as u8;

            moves.push((from, to));

            source &= source - 1;
            destination &= destination - 1;
        }

        moves
    }
}
