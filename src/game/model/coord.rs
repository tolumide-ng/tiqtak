#[derive(Debug, Clone, Copy)]
pub(crate) enum Coord {
    /// Top Left
    NorthWest,
    /// Top Right
    NorthEast,
    /// Bottom Left
    SouthWest,
    /// Bottom Right
    SouthEast,
}

impl Coord {
    /// Returns the shamt (Shift amount)
    pub(crate) fn shamt(&self, row: u8) -> Option<u8> {
        if row > 7 {
            return None;
        }

        let is_even = row % 2 == 0;

        match (self, is_even) {
            (Self::NorthEast, true) => Some(3),
            (Self::NorthWest, true) => Some(4),
            (Self::SouthEast, true) => Some(5),
            (Self::SouthWest, true) => Some(4),
            (Self::NorthEast, false) => Some(4),
            (Self::NorthWest, false) => Some(5),
            (Self::SouthEast, false) => Some(4),
            (Self::SouthWest, false) => Some(3),
        }
    }
}
