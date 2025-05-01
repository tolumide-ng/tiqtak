use std::{fmt::Display, ops::Deref};

use super::action::Action;
use crate::mcts::traits::Action as MctsAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ActionPath {
    pub(crate) mvs: [u16; 20],
    pub(crate) len: usize,
}

impl MctsAction for ActionPath {}

impl Display for ActionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", &self.mvs[..self.len])
    }
}

impl ActionPath {
    pub(crate) fn new() -> Self {
        Self {
            mvs: [0u16; 20],
            len: 0,
        }
    }

    pub(crate) fn append(&mut self, mv: Action) {
        self.mvs[self.len] = mv.into();
        self.len += 1;
    }

    pub(crate) fn prepend(&mut self, mv: Action) {
        assert!(self.len < 20, "ActionPath overflow");

        self.mvs.copy_within(0..self.len, 1);
        self.mvs[0] = mv.into();
        self.len += 1;
    }

    pub(crate) fn all_captures(&self) -> bool {
        self.mvs[..self.len]
            .iter()
            .all(|x| Action::from(*x).capture)
    }

    pub(crate) fn peek(&self, index: usize) -> Option<Action> {
        if index > self.len {
            return None;
        }

        return Some(self.mvs[index].into());
    }
}

impl From<Action> for ActionPath {
    fn from(value: Action) -> Self {
        let mut result = Self {
            mvs: [0; 20],
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
