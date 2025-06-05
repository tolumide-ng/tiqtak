use crate::{Player, game::traits::u32_shift::U32Ext};

pub struct Bits(u32);

impl Bits {
    pub(crate) fn new(n: u32) -> Self {
        Self(n)
    }
}

impl AsRef<u32> for Bits {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsMut<u32> for Bits {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl From<u32> for Bits {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl U32Ext for Bits {
    fn shift_by(&self, shift: u8, player: Player) -> u32 {
        match player {
            Player::South => self.0 << shift,
            Player::North => self.0 >> shift,
        }
    }
}

impl Iterator for Bits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let idx = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(idx)
    }
}
