use crate::game::model::player::Player;

pub(crate) trait U32Ext {
    fn shift_by(&self, shift: u8, player: Player) -> u32;
}

impl U32Ext for u32 {
    fn shift_by(&self, shift: u8, player: Player) -> u32 {
        match player {
            Player::South => self << shift,
            Player::North => self >> shift,
        }
    }
}
