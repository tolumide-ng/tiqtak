use std::{fmt::Display, ops::Index};

use crate::game::{bitboard::BitBoard, utils::Player};

use super::action::Action;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Board {
    /// white pieces and white kings
    pub(crate) north: u64,
    /// black pieces pieces and black kings
    pub(crate) south: u64,
    /// black and white kings
    pub(crate) kings: u64,
    /// 0 is for first player, and 1 is for bottom player
    pub(crate) turn: Player,
}

impl Board {
    pub(crate) fn new() -> Self {
        let north: u64 = 0xaa55aa0000000000;
        let south: u64 = 0x55aa55;

        Self {
            north,
            south,
            kings: 0,
            turn: Player::South,
        }
    }

    // to get the left move exclude any piece that is already on column A
    // to get the right move exclude any piece that is already on column H

    // to get the bottom moves exclude any piece that is already on row 1
    // to get the top moves (whichever direction) exclude any piece that is already on row 8

    /// returns the positions of the kings of the provided color on the board
    fn kings(&self, player: Player) -> u64 {
        match player {
            Player::North => self.north & self.kings,
            Player::South => self.south & self.kings,
        }
    }

    /// Returns the positions of the regular members for a specific color, excluding the kings on the board
    fn regular(&self, player: Player) -> u64 {
        match player {
            Player::North => self.north & !self.kings,
            Player::South => self.south & !self.kings,
        }
    }

    // todo: we need to establish and differentiate between what is a capture and isn't

    /// returns all the possible options a selected piece can play
    /// Vec<(from, to, capture)>
    pub(crate) fn options(&self, turn: Player) -> Vec<Action> {
        let regulars = self.regular(turn);
        let kings = self.kings(turn);

        let opponent = self[!turn];
        let mut natural_mvs = BitBoard::from((regulars | kings, opponent)).moves(turn);
        let mut king_mvs = BitBoard::from((kings, opponent)).moves(!turn); // extra king moves

        natural_mvs.reserve(king_mvs.len());
        natural_mvs.append(&mut king_mvs);
        natural_mvs
    }

    pub(crate) fn play(&self, mv: Action) -> Option<Self> {
        let options = self.options(self.turn);
        let valid = options
            .iter()
            .find(|op| op.src == mv.src && op.tgt == mv.tgt);

        let Some(Action { src, tgt, capture }) = valid else {
            return None;
        };

        let (north, south, kings) = match self.turn {
            Player::North => {
                let north = (self.north & !(1 << src)) | 1 << tgt; // we remove the piece from src (and then add it to the target (|...))
                let south = self.south & !((*capture as u64) << tgt);

                let is_king = (BitBoard::BOTTOM & (1 << tgt)) != 0;
                let kings = self.kings | ((is_king as u64) << tgt);

                (north, south, kings)
            }
            Player::South => {
                let south = (self.south & !(1 << src)) | 1 << tgt;
                let north = self.north & !((*capture as u64) << tgt);

                let is_king = (BitBoard::TOP & (1 << tgt)) != 0;
                let kings = self.kings | ((is_king as u64) << tgt);

                (north, south, kings)
            }
        };

        return Some(Self {
            north,
            south,
            kings,
            turn: !self.turn,
        });
    }
}

impl Index<Player> for Board {
    type Output = u64;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::North => &self.north,
            Player::South => &self.south,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dimension = 8;

        writeln!(f, "-------------------------------------------------")?;
        // writeln!(f, "")?;
        for row in (0..8).rev() {
            for col in 0..8 {
                let index = (row * dimension) + col;
                let cell = 1u64 << index;
                let king = (cell & self.kings) != 0; // whether or not this piece is a king

                let bp = (cell & self.south) != 0; // black piece (southern)
                let wp = (cell & self.north) != 0; // white piece (northern)

                let piece = match (bp, wp, king) {
                    (true, false, false) => "B",
                    (true, false, true) => "BK",
                    (false, true, false) => "W",
                    (false, true, true) => "WK",
                    _ => "",
                };

                if col == 0 {
                    write!(f, "|")?;
                }
                write!(f, " {:^3} |", piece)?;
            }
            writeln!(f, "")?;
            writeln!(f, "-------------------------------------------------")?;
            writeln!(f, "Turn: {:?}", self.turn)?;
        }

        writeln!(f, "checkers board")?;

        Ok(())
    }
}
