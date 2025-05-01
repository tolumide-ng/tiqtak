use std::fmt::Display;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::mcts::traits::Action as MctsAction;

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    pub(crate) src: u8,
    pub(crate) tgt: u8,
    pub(crate) capture: bool,
    pub(crate) promoted: bool,
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl From<(u8, u8, bool, bool)> for Action {
    fn from((src, tgt, capture, promoted): (u8, u8, bool, bool)) -> Self {
        Self {
            src,
            tgt,
            capture,
            promoted,
        }
    }
}

const SHIFT_SRC: u8 = 8;
const SHIFT_TGT: u8 = 2;
const SHIFT_CP: u8 = 1; // shift_capture

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Action {
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new(src: u8, tgt: u8, capture: bool, promoted: bool) -> Self {
        Self {
            src,
            tgt,
            capture,
            promoted,
        }
    }
}

impl MctsAction for Action {}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = ('A'..='H').into_iter().collect::<Vec<_>>();
        let src_col = (self.src % 8) as usize;
        let src_row = (self.src / 8) + 1;
        let tgt_col = (self.tgt % 8) as usize;
        let tgt_row = (self.tgt / 8) + 1;

        write!(
            f,
            "{{src: {:?}{}, tgt: {:?}{}, capture: {:?}, promoted: {:?}}}",
            src_row, cols[src_col], tgt_row, cols[tgt_col], self.capture, self.promoted
        )?;

        Ok(())
    }
}

impl From<Action> for u16 {
    fn from(value: Action) -> Self {
        let result = (u16::from(value.src) << SHIFT_SRC)
            | (u16::from(value.tgt) << SHIFT_TGT)
            | (u16::from(value.capture) << SHIFT_CP)
            | u16::from(value.promoted);

        result
    }
}

impl From<u16> for Action {
    fn from(value: u16) -> Self {
        let src = ((value >> SHIFT_SRC) & 0b0011_1111) as u8;
        let tgt = ((value >> SHIFT_TGT) & 0b0011_1111) as u8;
        let capture = ((value >> SHIFT_CP) & 1) != 0;
        let promoted = (value & 0b0000_0001) != 0;

        Self {
            src,
            tgt,
            capture,
            promoted,
        }
    }
}

#[cfg(test)]
mod action {
    use super::Action;

    #[test]
    fn should_create_and_destructure_action() {
        let action = Action::from((9, 18, true, false));
        assert_eq!(action.src, 9);
        assert_eq!(action.tgt, 18);
        assert_eq!(action.capture, true);
        assert_eq!(action.promoted, false);

        let action_u16 = u16::from(action);
        let new_action = Action::from(action_u16);

        assert_eq!(new_action.src, 9);
        assert_eq!(new_action.tgt, 18);
        assert_eq!(new_action.capture, true);
        assert_eq!(new_action.promoted, false);
    }

    #[test]
    fn should_properly_destructure_more_actions() {
        let action = Action::from((18, 23, false, true));
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
