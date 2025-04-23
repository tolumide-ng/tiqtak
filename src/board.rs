use crate::{bitboard::BitBoard, direction::Player};

pub(crate) struct Board {
    /// white pieces and white kings
    north: BitBoard,
    /// black pieces pieces and black kings
    south: BitBoard,
    /// black and white kings
    kings: BitBoard,
    /// 0 is for first player, and 1 is for bottom player
    turn: Player,
}

impl Board {
    // to get the left move exclude any piece that is already on column A
    // to get the right move exclude any piece that is already on column H

    // to get the bottom moves exclude any piece that is already on row 1
    // to get the top moves (whichever direction) exclude any piece that is already on row 8

    /// returns the positions of the kings of the provided color on the board
    fn kings(&self, player: Player) -> u64 {
        match player {
            Player::North => *self.north & *self.kings,
            Player::South => *self.south & *self.kings,
        }
    }

    /// Returns the positions of the regular members for a specific color, excluding the kings on the board
    fn regular(&self, player: Player) -> u64 {
        match player {
            Player::North => *self.north & !*self.kings,
            Player::South => *self.south & !*self.kings,
        }
    }

    // todo: we need to establish and differentiate between what is a capture and isn't

    /// returns all the possible options a selected piece can play
    pub(crate) fn options(&self, turn: Player) -> Vec<(u8, u8)> {
        let regulars = self.regular(turn);
        let kings = self.kings(turn);

        let mut natural_mvs = BitBoard::from(regulars | kings).moves(turn);
        let mut king_mvs = BitBoard::from(kings).moves(!turn); // extra king moves

        natural_mvs.reserve(king_mvs.len());
        natural_mvs.append(&mut king_mvs);
        natural_mvs
    }
}
