use crate::game::model::player::Player;

pub(crate) trait U64Ext {
    fn shift_by(&self, shift: u8, player: Player) -> u64;
}

impl U64Ext for u64 {
    fn shift_by(&self, shift: u8, player: Player) -> u64 {
        match player {
            Player::South => self << shift,
            Player::North => self >> shift,
        }
    }
}
