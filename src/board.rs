#[derive(Debug)]
pub(crate) enum Color {
    White,
    Black,
}

pub(crate) struct Board {
    /// white pieces and white kings
    white: u32,
    /// black pieces pieces and black kings
    black: u32,
    /// black and white kings
    kings: u32,
    turn: Color,
}

impl Board {
    /// returns the positions of the kings of the provided color on the board
    pub(crate) fn kings(&self, color: Color) -> u32 {
        match color {
            Color::Black => self.black & self.kings,
            Color::White => self.white & self.kings,
        }
    }

    /// Returns the positions of the regular members for a specific color, excluding the kings on the board
    pub(crate) fn regular(&self, color: Color) -> u32 {
        match color {
            Color::Black => self.black & !self.kings,
            Color::White => self.white & !self.kings,
        }
    }

    /// returns all the possible options a selected piece can play
    pub(crate) fn options(&self, color: Color, piece: u32) {}
}
