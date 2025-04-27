use std::fmt::Display;

use crate::mcts::traits::Action as MctsAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Action {
    pub(crate) src: u8,
    pub(crate) tgt: u8,
    pub(crate) capture: bool,
    pub(crate) promoted: bool,
}

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

impl Action {
    const SHIFT_SRC: u8 = 8;
    const SHIFT_TGT: u8 = 2;
    const SHIFT_CP: u8 = 1; // shift_capture

    pub(crate) fn new(src: u8, tgt: u8, capture: bool, promoted: bool) -> Self {
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

impl From<Action> for u16 {
    fn from(value: Action) -> Self {
        let result = (u16::from(value.src) << Action::SHIFT_SRC)
            | (u16::from(value.tgt) << Action::SHIFT_TGT)
            | (u16::from(value.capture) << Action::SHIFT_CP)
            | u16::from(value.promoted);

        result
    }
}

impl From<u16> for Action {
    fn from(value: u16) -> Self {
        let src = ((value >> Action::SHIFT_SRC) & 0b0011_1111) as u8;
        let tgt = ((value >> Action::SHIFT_TGT) & 0b0011_1111) as u8;
        let capture = ((value >> Action::SHIFT_CP) & 0b0000_0010) != 0;
        let promoted = (value & 0b0000_0001) != 0;

        Self {
            src,
            tgt,
            capture,
            promoted,
        }
    }
}
