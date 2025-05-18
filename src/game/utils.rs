use std::ops::{Index, IndexMut};

use thiserror::Error;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::mcts::traits::MCTSError;

use super::model::player::Player;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ApiError {
    #[error("Illegal move")]
    IllegalMove,
}

impl MCTSError for ApiError {}

/// Number of quiet moves per player
#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
