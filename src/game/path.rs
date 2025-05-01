use std::{fmt::Display, ops::Deref};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use super::action::Action;
use crate::mcts::traits::Action as MctsAction;

const LEN: usize = 12;

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
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            mvs: [0u16; LEN],
            len: 0,
        }
    }

    pub fn append(&mut self, mv: Action) {
        self.mvs[self.len] = mv.into();
        self.len += 1;
    }

    pub fn prepend(&mut self, mv: Action) {
        assert!(self.len < LEN, "ActionPath overflow");

        self.mvs.copy_within(0..self.len, 1);
        self.mvs[0] = mv.into();
        self.len += 1;
    }

    pub fn peek(&self, index: usize) -> Option<Action> {
        if index > self.len {
            return None;
        }

        return Some(self.mvs[index].into());
    }
}

#[cfg_attr(feature = "web", wasm_bindgen)]
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

#[cfg_attr(feature = "web", wasm_bindgen)]
impl From<&[u16]> for ActionPath {
    #[cfg_attr(feature = "web", wasm_bindgen)]
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
