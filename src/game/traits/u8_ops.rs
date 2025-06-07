use crate::Player;

use super::u32_shift::U32Ext;

pub(crate) trait U8Ext {
    // const ROW_8_MASK: u32 = 1 << 28 | 1 << 29 | 1 << 30 | 1 << 31;
    // const ROW_1_MASK: u32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;

    /// By how many cells/squares a piece should move on the bitboard
    fn move_by(&self, by: u8, player: Player) -> u8;

    // fn promoted(&self, tgt: u8, player: Player) -> bool;
}

impl U8Ext for u8 {
    fn move_by(&self, by: u8, player: Player) -> u8 {
        match player {
            Player::North => self - by,
            Player::South => self + by,
        }
    }

    // fn promoted(&self, tgt: u8, player: Player) -> bool {
    //     match player {
    //         Player::North => {
    //             let pos = (1u32).shift_by(tgt, player);
    //         }
    //         Player::South => {}
    //     }

    //     false
    // }
}
