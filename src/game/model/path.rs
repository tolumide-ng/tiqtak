use std::{fmt::Display, ops::Deref};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::game::model::action::Action;
use crate::mcts::traits::Action as MctsAction;

const LEN: usize = 12;

/// A list of action the user intends to play, in a scenario where there is no jump move
/// this would only be one move(Action)
/// ```rust
/// use tiktaq::game::model::{action::Action, path::ActionPath};
/// let mut mv = ActionPath::new(); // creates an empty
/// mv.append(Action::new(8, 32, true, false)); // adds this to the mv list
/// mv.prepend(Action::from((48, 32, true, false))); // Reserves the original order of the moves, but adds this as the first move, followed by the existing ones
/// mv.append(Action::new(8, 2, false, true)); // append to the moves list
/// // final path would look like 16(src) -> 48(src) -> 32(target) -> 8 -> 2
/// ```
#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionPath {
    pub(crate) mvs: [u16; LEN],
    pub(crate) len: usize,
}

impl MctsAction for ActionPath {}

impl Display for ActionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", &self.mvs[..self.len])
    }
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl ActionPath {
    /// Creates a new ActionPath with no move in it at all
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            mvs: [0u16; LEN],
            len: 0,
        }
    }

    /// Appends an action (move) to the actionPath
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn append(&mut self, mv: Action) {
        self.mvs[self.len] = mv.into();
        self.len += 1;
    }

    /// Prepends an action,
    /// presists the existing order of the moves, but automatically makes this new move the first move  
    /// followed by the existing moves already on this path
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn prepend(&mut self, mv: Action) {
        assert!(self.len < LEN, "ActionPath overflow");

        self.mvs.copy_within(0..self.len, 1);
        self.mvs[0] = mv.into();
        self.len += 1;
    }

    /// Returns the move(Action) at the `index` position of this actionPath  
    /// returns None if there is no move at that index
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn peek(&self, index: usize) -> Option<Action> {
        if index > self.len {
            return None;
        }

        return Some(self.mvs[index].into());
    }
}

impl From<Action> for ActionPath {
    fn from(value: Action) -> Self {
        let mut result = Self {
            mvs: [0; LEN],
            len: 0,
        };

        result.append(value);
        result
    }
}

impl From<&[u16]> for ActionPath {
    fn from(value: &[u16]) -> Self {
        let mut path = Self::new();
        path.mvs[..value.len()].copy_from_slice(value);
        path.len = value.len();
        path
    }
}

impl Deref for ActionPath {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.mvs[..self.len]
    }
}
