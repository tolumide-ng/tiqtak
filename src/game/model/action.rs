use std::fmt::Display;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::{game::board::scale::Scale, mcts::traits::Action as MctsAction};

/**
 * 0000 0000 0000 0000
 * 0000 0000 0011 1111 ->  src  
 * 0000 1111 1100 0000 -> tgt  
 * 0001 0000 0000 0000 -> captured  
 * 0010 0000 0000 0000 -> promoted  
 * 0100 0000 0000 0000 -> this bin is u64 board format  
 * A specific move on the checkers board
 */
#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    pub(crate) src: u8,
    pub(crate) tgt: u8,
    pub(crate) capture: bool,
    pub(crate) promoted: bool,
    pub(crate) scale: Scale,
    // todo! can we store whether this struct is a u64, or u32 in this struct and bin pack?
}

type ActionTuple<T> = (u8, u8, bool, bool, T);

impl From<ActionTuple<bool>> for Action {
    fn from((src, tgt, capture, promoted, scale): ActionTuple<bool>) -> Self {
        Self::new(src, tgt, capture, promoted, Scale::from(scale))
    }
}

impl From<ActionTuple<Scale>> for Action {
    fn from((src, tgt, capture, promoted, scale): ActionTuple<Scale>) -> Self {
        Self::new(src, tgt, capture, promoted, scale)
    }
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Action {
    const SRC_MASK: u16 = 0b0011_1111;
    const TGT_MASK: u16 = 0b1111_1100_0000;

    const SHIFT_TGT: u8 = 6;
    const SHIFT_CP: u8 = 12; // shift capture
    const SHIFT_P: u8 = 13; // shift promoted
    const SHIFT_BITS: u8 = 14; // shift -> bits format (e.g u64 bitboard or u32 bitboard)

    /// Creates a new Action(move) for the checkers board  
    /// src: represents the position of the piece that would be moved  
    /// tgt: represents the target position where this piece would be placed after the move  
    /// capture: Whether or not this move would be capturing the opponent's piece on the board  
    /// promoted: Whether or not this move would result in the promotion of the moving(this) piece
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new(src: u8, tgt: u8, capture: bool, promoted: bool, scale: Scale) -> Self {
        Self {
            src,
            tgt,
            capture,
            promoted,
            scale,
        }
    }

    /// Creates an Action for a checkers bitboard (32 bits bitboard)
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new_32(src: u8, tgt: u8, capture: bool, promoted: bool) -> Self {
        Self {
            src,
            tgt,
            capture,
            promoted,
            scale: Scale::U32,
        }
    }

    /// Converts a u32 format Action to u64, and a u64 format of Action to u32
    /// NB: The term u64 or u32 refers to the actual mapping of the board.
    pub fn transcode(&self) -> Self {
        let Action {
            src,
            tgt,
            capture,
            promoted,
            scale,
        } = *self;

        match self.scale {
            Scale::U32 => {
                let cols = 4; // number of columns per row in 32bits board
                // whether the row of the src/tgt is an even numbered (rows are numbered from 0 to 7)
                let (src_even, tgt_even) = ((src / cols) % 2 == 0, (tgt / cols) % 2 == 0);
                let src = (src * 2) + !(src_even) as u8;
                let tgt = (tgt * 2) + !(tgt_even) as u8;
                Action::new(src, tgt, capture, promoted, scale)
            }

            Scale::U64 => Action::new(src / 2, tgt / 2, capture, promoted, Scale::U32),
        }
    }
}

impl MctsAction for Action {}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = *self;

        if self.scale == Scale::U32 {
            value = self.transcode();
        }

        let cols = ('A'..='H').into_iter().collect::<Vec<_>>();
        let src_col = (value.src % 8) as usize;
        let src_row = (value.src / 8) + 1;
        let tgt_col = (value.tgt % 8) as usize;
        let tgt_row = (value.tgt / 8) + 1;

        write!(
            f,
            "{{src: {:?}{}, tgt: {:?}{}, capture: {:?}, promoted: {:?}, scale: {:?}}}",
            src_row,
            cols[src_col],
            tgt_row,
            cols[tgt_col],
            value.capture,
            value.promoted,
            value.scale
        )?;

        Ok(())
    }
}

/// TODO! NEED TO IMPROVE THIS FURTHER PLEASE!!
/// THE MAX NUMBER OF BITS IN THE DECIMAL 63 (MAX POSSIBLE CELL IN A 0 INDEXED 64 BIT BITBOARD) IS 6BITS
/// HOW?
/// from lsb to msb (i.e msb <- lsb)
/// first 6 bits - src
/// next 6 bits - tgt
/// next 1 bit - captured
/// next 1 bit - prompted
/// next 1 bit - whether this action is in u32 or u64 format for the squares
///     if its u64 bit should be set to 1
///     if its u32 bit should be set to 0
/// last 1 bit - free for now
impl From<Action> for u16 {
    fn from(value: Action) -> Self {
        let result = ((value.scale as u16) << Action::SHIFT_BITS)
            | (u16::from(value.promoted) << Action::SHIFT_P)
            | (u16::from(value.capture) << Action::SHIFT_CP)
            | (u16::from(value.tgt) << Action::SHIFT_TGT)
            | (u16::from(value.src));

        result
    }
}

impl From<u16> for Action {
    fn from(value: u16) -> Self {
        let src = (value & Self::SRC_MASK) as u8;
        let tgt = ((value & Self::TGT_MASK) >> Self::SHIFT_TGT) as u8;
        let capture = (value & (1 << Self::SHIFT_CP)) != 0;
        let promoted = (value & (1 << Self::SHIFT_P)) != 0;
        let is_u64 = (value & (1 << Self::SHIFT_BITS)) != 0;

        Self {
            src,
            tgt,
            capture,
            promoted,
            scale: Scale::from(is_u64),
        }
    }
}

#[cfg(test)]
mod action {
    use crate::game::board::scale::Scale;

    use super::Action;

    #[test]
    fn should_create_and_destructure_action() {
        let action = Action::from((9, 18, true, false, true));
        assert_eq!(action.src, 9);
        assert_eq!(action.tgt, 18);
        assert_eq!(action.capture, true);
        assert_eq!(action.promoted, false);
        assert_eq!(action.scale, Scale::U64);

        let action_u16 = u16::from(action);
        let new_action = Action::from(action_u16);

        assert_eq!(new_action.src, 9);
        assert_eq!(new_action.scale, Scale::U64);
        assert_eq!(new_action.tgt, 18);
        assert_eq!(new_action.capture, true);
        assert_eq!(new_action.promoted, false);
    }

    #[test]
    fn should_properly_destructure_more_actions() {
        let action = Action::from((18, 23, false, true, true));
        assert_eq!(action.src, 18);
        assert_eq!(action.tgt, 23);
        assert_eq!(action.capture, false);
        assert_eq!(action.promoted, true);

        let action_u16 = u16::from(action);
        let new_action = Action::from(action_u16);

        assert_eq!(new_action.src, 18);
        assert_eq!(new_action.tgt, 23);
        assert_eq!(new_action.capture, false);
        assert_eq!(new_action.promoted, true);
    }
}
