use crate::mcts::traits::Action as MctsAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Action {
    pub(crate) src: u8,
    pub(crate) tgt: u8,
    pub(crate) capture: bool,
}

impl Action {
    pub(crate) fn new(src: u8, tgt: u8, capture: bool) -> Self {
        Self { src, tgt, capture }
    }
}

impl MctsAction for Action {}
