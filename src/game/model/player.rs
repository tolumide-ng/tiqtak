use std::ops::Not;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::mcts::traits::Player as PlayerTrait;

/// The only two players for the checkers game  
/// North: represents the player on the northern part of the board
/// South: represents the player on the southern part of the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "web", wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Player {
    North = 0,
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

impl From<Player> for usize {
    fn from(value: Player) -> Self {
        match value {
            Player::North => 0,
            Player::South => 1,
        }
    }
}
