use crate::bitboard::BitBoard;

#[derive(Debug)]
pub(crate) enum Color {
    White,
    Black,
}

pub(crate) struct Board {
    /// white pieces and white kings
    white: BitBoard,
    /// black pieces pieces and black kings
    black: BitBoard,
    /// black and white kings
    kings: BitBoard,
    /// 0 is for first player, and 1 is for bottom player
    turn: bool,
}

impl Board {
    // to get the left move exclude any piece that is already on column A
    // to get the right move exclude any piece that is already on column H

    // to get the bottom moves exclude any piece that is already on row 1
    // to get the top moves (whichever direction) exclude any piece that is already on row 8

    /// returns the positions of the kings of the provided color on the board
    pub(crate) fn kings(&self, color: Color) -> u64 {
        match color {
            Color::Black => *self.black & *self.kings,
            Color::White => *self.white & *self.kings,
        }
    }

    /// Returns the positions of the regular members for a specific color, excluding the kings on the board
    pub(crate) fn regular(&self, color: Color) -> u64 {
        match color {
            Color::Black => *self.black & !*self.kings,
            Color::White => *self.white & !*self.kings,
        }
    }

    /// returns all the possible options a selected piece can play
    pub(crate) fn options(&self, color: Color, piece: u32) {}
}
