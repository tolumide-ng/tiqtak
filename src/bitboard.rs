use std::ops::{Deref, DerefMut};

pub(crate) struct BitBoard(u64);

impl BitBoard {
    pub(crate) const LEFT: u64 = 0x101010101010101;
    pub(crate) const RIGHT: u64 = 0x8080808080808080;
    pub(crate) const BOTTOM: u64 = 0xff;
    pub(crate) const TOP: u64 = 0xff00000000000000;
}

impl Deref for BitBoard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
