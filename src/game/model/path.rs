use std::{fmt::Display, ops::Deref};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::game::board::scale::Scale;
use crate::game::model::action::Action;
use crate::game::utils::ApiError;
use crate::mcts::traits::Action as MctsAction;

const LEN: usize = 12;

/// A list of action the user intends to play, in a scenario where there is no jump move
/// this would only be one move(Action)
/// ```rust
/// use tiqtak::{Action, ActionPath, Scale};
/// let mut mv = ActionPath::new(Scale::U64); // creates an empty
/// mv.append(Action::new(8, 32, true, false, Scale::U64)); // adds this to the mv list
/// mv.prepend(Action::from((48u8, 32u8, true, false, Scale::U64))); // Reserves the original order of the moves, but adds this as the first move, followed by the existing ones
/// mv.append(Action::new(8, 2, false, true, Scale::U64)); // append to the moves list
/// // final path would look like 16(src) -> 48(src) -> 32(target) -> 8 -> 2
/// ```
#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionPath {
    pub(crate) mvs: [u16; LEN],
    pub(crate) len: usize,
    pub(crate) scale: Scale,
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
    pub fn new(scale: Scale) -> Self {
        Self {
            mvs: [0u16; LEN],
            len: 0,
            scale,
        }
    }

    /// Update the ActionPath to register the variant of action stored (only one format should be used for all the actions in this path)
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn variant(&mut self, s: Scale) {
        self.scale = s;
    }

    /// Update the ActionPath to register the variant of action stored
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn is_u64(&self) -> bool {
        self.scale == Scale::U64
    }

    pub(crate) fn parse(self, mv: Action) -> Result<Action, ApiError> {
        if self.len > 0 && mv.scale != self.scale {
            return Err(ApiError::IncompatibleActions);
        }
        Ok(mv)
    }

    pub(crate) fn parse_me(self) -> Result<Self, ApiError> {
        self.mvs
            .iter()
            .try_for_each(|mv| self.parse((*mv).into()).map(drop))?;

        Ok(self)
    }

    /// Appends an action (move) to the actionPath
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn append(&mut self, mv: Action) -> Result<(), ApiError> {
        self.mvs[self.len] = self.parse(mv)?.into();
        self.len += 1;

        Ok(())
    }

    /// Prepends an action,
    /// presists the existing order of the moves, but automatically makes this new move the first move  
    /// followed by the existing moves already on this path
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn prepend(&mut self, mv: Action) -> Result<(), ApiError> {
        assert!(self.len < LEN, "ActionPath overflow");
        let mv = self.parse(mv)?;

        self.mvs.copy_within(0..self.len, 1);
        self.mvs[0] = mv.into();
        self.len += 1;

        Ok(())
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
        let mut result = Self::new(value.scale);
        let _ = result.append(value).unwrap();
        result
    }
}

// modify this to a tryfrom, so that it breaks if the actions are not of the same type
impl TryFrom<&[u16]> for ActionPath {
    type Error = ApiError;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        let act = Action::from(*value.get(0).unwrap_or(&0));

        let mut path = Self::new(act.scale);
        path.mvs[..value.len()].copy_from_slice(value);
        path.len = value.len();

        path.parse_me()
    }
}

impl Deref for ActionPath {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.mvs[..self.len]
    }
}

// modify this to a tryfrom, so that it breaks if the actions are not of the same type
impl From<ActionPath> for String {
    fn from(value: ActionPath) -> Self {
        let mut result = format!("");

        for (index, action) in (&value[..value.len]).iter().enumerate() {
            result.push_str(&action.to_string());
            if index != value.len() - 1 {
                result.push('*');
            }
        }

        result
    }
}

impl TryFrom<String> for ActionPath {
    type Error = ApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut result = Self::new(Scale::U64);
        let parts = value.split("*");

        for (i, ac) in parts.into_iter().enumerate() {
            let action = ac.parse::<u16>().map_err(|_| ApiError::IllegalMove)?;
            if i > LEN {
                return Err(ApiError::TooManyActions);
            }

            let scale = Action::from(action).scale;
            if i == 0 {
                result.scale = scale;
            } else if scale != result.scale {
                return Err(ApiError::IncompatibleActions);
            }

            result.mvs[result.len] = action;
            result.len += 1;
        }

        Ok(result)
    }
}
