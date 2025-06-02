use crate::Board;

impl Board {
    const L3_MASK: u32 = 0xE0E0E0E;
    const L5_MASK: u32 = 0x707070;
    const R3_MASK: u32 = 0x70707070;
    const R5_MASK: u32 = 0xE0E0E00;

    pub(crate) fn white_movers(&self) -> u32 {
        let empty = !(self.north | self.south);
        let wk = self.north & self.kings;

        let mut movers = 0;

        let mut temp = (empty << 4) & self.south;
        if temp != 0 {
            movers |= (((temp & Self::L3_MASK) << 3) | ((temp & Self::L5_MASK) << 5)) & self.north;
        }

        temp = (((empty & Self::L3_MASK) << 3) | ((empty & Self::L5_MASK) << 5)) & self.south;
        movers |= (temp << 4) & self.north;

        if self.north != 0 {
            temp = (empty >> 4) & self.north;
            if temp != 0 {
                movers |= (((temp & Self::R3_MASK) >> 3) | ((temp & Self::R5_MASK) >> 5)) & wk;
            }
            temp = (((empty & Self::R3_MASK) >> 3) | ((empty & Self::R5_MASK) >> 5)) & self.south;
            if temp != 0 {
                movers |= (temp >> 4) & self.north;
            }
        }

        movers
    }
}
