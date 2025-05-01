use std::ops::{Index, IndexMut, Not};

use thiserror::Error;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::mcts::traits::{MCTSError, Player as PlayerTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub enum Player {
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

/// Number of quiet moves per player
#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Qmvs {
    pub(crate) north: u8,
    pub(crate) south: u8,
}

impl Index<Player> for Qmvs {
    type Output = u8;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::North => &self.north,
            Player::South => &self.south,
        }
    }
}

impl IndexMut<Player> for Qmvs {
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        match index {
            Player::North => &mut self.north,
            Player::South => &mut self.south,
        }
    }
}
