use crate::Action;

pub enum Play {
    U32(Action),
    U64(Action),
}

impl Play {
    pub fn transcode(&self) -> Self {
        match self {
            Self::U32(act) => {
                let Action { src, tgt, .. } = act;
                let cols = 4; // number of columns per row in 32bits board

                // whether the row of the src/tgt is an even numbered (rows are numbered from 0 to 7)
                let (src_even, tgt_even) = ((*src / cols) % 2 == 0, (*tgt / cols) % 2 == 0);

                let src = (src * 2) + !(src_even as u8);
                let tgt = (tgt * 2) + !(tgt_even as u8);

                Self::U64(Action {
                    src,
                    tgt,
                    capture: act.capture,
                    promoted: act.promoted,
                })
            }
            Self::U64(act) => {
                let Action { src, tgt, .. } = act;
                Self::U32(Action {
                    src: src / 2,
                    tgt: tgt / 2,
                    capture: act.capture,
                    promoted: act.promoted,
                })
            }
        }
    }
}
