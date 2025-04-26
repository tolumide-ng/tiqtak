use std::fmt::Display;

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

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = ('A'..'H').into_iter().collect::<Vec<_>>();
        let src_col = (self.src % 8) as usize;
        let src_row = (self.src / 8) + 1;
        let tgt_col = (self.tgt % 8) as usize;
        let tgt_row = (self.tgt / 8) + 1;

        write!(
            f,
            "{{src: {:?}{}, tgt: {:?}{}}}",
            src_row, cols[src_col], tgt_row, cols[tgt_col]
        )?;

        Ok(())
    }
}
