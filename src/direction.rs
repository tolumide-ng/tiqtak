use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Player {
    North,
    South,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
        }
    }
}
