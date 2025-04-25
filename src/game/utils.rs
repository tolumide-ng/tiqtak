use std::ops::Not;

use thiserror::Error;

use crate::mcts::traits::{MCTSError, Player as PlayerTrait};

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

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("Illegal move")]
    IllegalMove,
}

impl MCTSError for AppError {}
