use std::fmt::Display;

use super::action::Action;
use crate::mcts::traits::Action as MctsAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ActionPath {
    pub(crate) mvs: [u16; 12],
    pub(crate) len: usize,
}

impl MctsAction for ActionPath {}

impl Display for ActionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", &self.mvs[..self.len])
    }
}

impl ActionPath {
    pub(crate) fn new(mvs: &[Action]) -> Self {
        Self {
            mvs: [0u16; 12],
            len: 0,
        }
    }

    pub(crate) fn append(&mut self, mv: Action) {
        self.mvs[self.len] = mv.into();
        self.len += 1;
    }

    pub(crate) fn prepend(&mut self, mv: Action) {
        self.mvs.copy_within(0.., 1);
        self.mvs[0] = mv.into();
    }
}
