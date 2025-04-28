use std::{collections::HashSet, fmt::Display, ops::Index};

use crate::game::{action::Action, bitboard::BitBoard, utils::Player};

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
    /// Quiet Moves (quite_mvs): The number of moves that's happened without a capture so far
    /// this value automatically resets to 0 for both sides after any capture.
    /// any of the values reaching 20 would result ina  "draw"
    pub(crate) qmvs: (u8, u8),
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
            qmvs: (0, 0),
        }
    }

    pub(crate) fn with(north: u64, south: u64, kings: u64, turn: Player, qmvs: (u8, u8)) -> Self {
        Self {
            north,
            south,
            kings,
            turn,
            qmvs,
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
        let mut natural_mvs = BitBoard::from((regulars | kings, opponent, 0)).moves(turn);
        let king_mvs = BitBoard::from((kings, opponent, regulars)).moves(!turn); // extra king moves

        // natural_mvs.reserve(king_mvs.len());
        // natural_mvs.append(&mut king_mvs);
        natural_mvs = &natural_mvs | &king_mvs;

        natural_mvs.into_iter().collect()
    }

    pub(crate) fn play(&self, mv: Action) -> Option<Self> {
        let options = self.options(self.turn);
        let valid = options
            .iter()
            .find(|op| op.src == mv.src && op.tgt == mv.tgt);

        let Some(Action {
            src, tgt, capture, ..
        }) = valid
        else {
            return None;
        };

        let (north, south, kings, qmvs) = match self.turn {
            Player::North => {
                // if the piece is a moving king, we ensure that they remain king no-matter where they move, by updating there position on king bin
                let kings = self.kings ^ (((self.kings >> src) & 1) * ((1 << src) | (1 << tgt)));
                let north = (self.north & !(1 << src)) | 1 << tgt; // we remove the piece from src (and then add it to the target (|...))
                let south = self.south & !((*capture as u64) << tgt);

                let kings = kings | ((mv.promoted as u64) << tgt);

                let cp = !(mv.capture) as u8;
                let qmvs = ((self.qmvs.0 + 1) * cp, self.qmvs.1 * cp);

                (north, south, kings, qmvs)
            }
            Player::South => {
                let kings = self.kings ^ (((self.kings >> src) & 1) * ((1 << src) | (1 << tgt)));
                let south = (self.south & !(1 << src)) | 1 << tgt;
                let north = self.north & !((*capture as u64) << tgt);

                let kings = kings | ((mv.promoted as u64) << tgt);

                let cp = (!mv.capture) as u8;
                let qmvs = (self.qmvs.0 * cp, (self.qmvs.1 + 1) * cp);

                (north, south, kings, qmvs)
            }
        };

        return Some(Self {
            north,
            south,
            kings,
            turn: !self.turn,
            qmvs,
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

        writeln!(f, "---------------------------------------------------")?;
        // writeln!(f, "")?;
        for row in (0..8).rev() {
            write!(f, "{} ", row + 1)?;
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
            writeln!(f, "---------------------------------------------------")?;
        }
        writeln!(f, "  |  A  |  B  |  C  |  D  |  E  |  F  |  G  |  H  | ")?;
        writeln!(f, "---------------------------------------------------")?;

        writeln!(f, "Turn: {:?}", self.turn)?;
        write!(f, "Quiet moves: {:?}", self.qmvs)?;
        writeln!(f, "checkers board")?;

        Ok(())
    }
}
