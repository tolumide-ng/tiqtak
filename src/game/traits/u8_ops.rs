use crate::Player;

pub(crate) trait U8Ext {
    /// By how many cells/squares a piece should move on the bitboard
    fn move_by(&self, by: u8, player: Player) -> u8;
}

impl U8Ext for u8 {
    fn move_by(&self, by: u8, player: Player) -> u8 {
        match player {
            Player::North => self - by,
            Player::South => self + by,
        }
    }
}
