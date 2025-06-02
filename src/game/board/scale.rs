use std::ops::Not;

/// Used for an action to tell whether the action belongs to a 64bits or 32bits bitboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scale {
    U64 = 1,
    U32,
}

impl From<bool> for Scale {
    fn from(value: bool) -> Self {
        match value {
            true => Scale::U64,
            false => Scale::U32,
        }
    }
}

impl Not for Scale {
    type Output = Scale;

    fn not(self) -> Self::Output {
        match self {
            Self::U32 => Self::U64,
            Self::U64 => Self::U32,
        }
    }
}
