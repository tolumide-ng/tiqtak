use std::ops::Not;

use crate::mcts::traits::Player as PlayerTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl PlayerTrait for Player {}
