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

impl Player {
    const L3_MASK: u32 = 0xE0E0E0E;
    const L5_MASK: u32 = 0x707070;
    const R3_MASK: u32 = 0x70707070;
    const R5_MASK: u32 = 0xE0E0E00;

    /// side 3 mask
    pub(crate) const fn s3_mask(&self) -> u32 {
        match self {
            Self::North => Self::L3_MASK,
            Self::South => Self::R3_MASK,
        }
    }

    /// side 5 mask
    pub(crate) const fn s5_mask(&self) -> u32 {
        match self {
            Self::North => Self::L5_MASK,
            Self::South => Self::R5_MASK,
        }
    }
}
