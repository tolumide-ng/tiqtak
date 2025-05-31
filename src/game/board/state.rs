#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Index};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::{
    game::{
        board::bitboard::BitBoard,
        model::{action::Action, path::ActionPath, player::Player},
        utils::{ApiError, Qmvs},
    },
    mcts::{
        algo::{state::State, tree_search::MCTS},
        utils::{limit::Limit, reward::Reward, skill_level::SkillLevel, strength::Strength},
    },
};

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Board {
    /// white pieces and white kings
    pub north: u32,
    /// black pieces pieces and black kings
    pub south: u32,
    /// black and white kings
    pub kings: u32,
    /// 0 is for first player, and 1 is for bottom player
    pub turn: Player,
    /// Quiet Moves (quite_mvs): The number of moves that's happened without a capture so far
    /// this value automatically resets to 0 for both sides after any capture.
    /// any of the values reaching 20 would result ina  "draw"
    pub qmvs: Qmvs,
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Board {
    /// Creates a brand new Checkers board with 12 pieces per team(player)
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new() -> Board {
        // let north: u64 = 0xaa55aa0000000000;
        // let south: u64 = 0x55aa55;

        let south: u32 = 0x00000FFF;
        let north: u32 = 0xFFF00000;

        Self {
            north,
            south,
            kings: 0,
            turn: Player::South,
            qmvs: Qmvs::default(),
        }
    }

    /// Creates a Checkers board using the provided information (args)
    /// e.g.  
    /// **north** -- the north team (the pieces available for the northern team),
    /// where each bit represents the position of the piece on the 64bits board  
    /// **south** -- u64 value representing the board and the (southern) pieces from the
    /// perspective of the southern player (this must include the kings of this piece)  
    /// **kings** -- u64 value representing the checkers board, and all the
    /// kings (both north and south) with each present king on the board represented by a set bit  
    /// **qmvs** - meaning quiet moves (qmvs), tracks the number of quiet moves since a captures by both
    /// players, this value automatically resets to (0, 0) if any of the players captures the opponent's piece
    /// If there is no capture after atleast 20 moves (from either player), the game automatically becomes a draw  
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn with(north: u32, south: u32, kings: u32, turn: Player, qmvs: Qmvs) -> Self {
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
    fn kings(&self, player: Player) -> u32 {
        match player {
            Player::North => self.north & self.kings,
            Player::South => self.south & self.kings,
        }
    }

    /// Returns the positions of the regular members for a specific color, excluding the kings on the board
    fn regular(&self, player: Player) -> u32 {
        match player {
            Player::North => self.north & !self.kings,
            Player::South => self.south & !self.kings,
        }
    }

    /// Checks whether the move (ActionPath) about to be played is valid based on the board's current state
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn is_valid(&self, action: ActionPath, turn: Player) -> bool {
        assert!(
            action.len > 0,
            "Invalid Action: There must be atleast one move in an action"
        );
        let mv = Action::from(action[0]);
        let src_mask = 1u32 << mv.src;

        if (self[turn] & src_mask) == 0 {
            return false;
        }

        let board = BitBoard::new(1 << mv.src, self[!turn], self[turn]);
        let mut moves = board.moves(turn);

        if self.kings & src_mask != 0 {
            moves.extend(board.moves(!turn));
        }

        moves.contains(&action)
    }

    /// Returns all the possible options(moves) that the selected user can play
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn options(&self, turn: Player) -> Vec<ActionPath> {
        let regulars = self.regular(turn);
        let kings = self.kings(turn);

        let opponent = self[!turn];
        let mut natural_mvs = BitBoard::from((regulars | kings, opponent, 0)).moves(turn);
        let king_mvs = BitBoard::from((kings, opponent, regulars)).moves(!turn); // extra king moves

        natural_mvs.reserve(king_mvs.len());
        natural_mvs.extend(king_mvs);

        natural_mvs
    }

    /// This returns a new Board state (the new board state) after the move (ActionPath) is applied to the board
    /// Please always provide only u64 format of the action for valid plays
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn play(&self, action: ActionPath) -> Option<Self> {
        if !self.is_valid(action, self.turn) {
            return None;
        }
        let mut board = *self;

        for mv in &action.mvs[..action.len] {
            let mut action = Action::from(*mv);

            if action.is_u64 {
                action = action.transcode();
            }

            let Action {
                src,
                tgt,
                capture,
                promoted,
                ..
            } = action;

            // println!("is >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> u64 {}", tgt);

            let src_mask = 1 << src;
            let tgt_mask = 1 << tgt;

            // if the piece is a moving king, we ensure that they remain king no-matter where they move, by updating there position on king bin
            let existing_kings = board.kings ^ (((board.kings >> src) & 1) * (src_mask | tgt_mask));
            let kings = existing_kings | ((promoted as u32) << tgt); // if the piece was just promoted, we add them to the list of kings
            let cp = !capture as u8;

            let turn = board.turn;
            let us = (board[turn] & !src_mask) | tgt_mask;
            let them = board[!turn] & !((capture as u32) << tgt);

            let (north, south) = match turn {
                Player::North => (us, them),
                Player::South => (them, us),
            };

            let mut qmvs = board.qmvs;
            qmvs[turn] = (qmvs[turn] + 1) * cp;
            qmvs[!turn] *= cp;

            board = Self::with(north, south, kings, turn, qmvs);
        }

        board.turn = !self.turn;
        return Some(board);
    }

    /// Generates the next best move based on the provided MCTS configuration
    /// NB: Only use this method when you're trying to get a bot's next move  
    /// exp: exploration constant for MCTS
    /// col: cost of losing (recommended ==> -1.25)
    /// limit: How long should MCTS think (in ms)? (recommended 100)
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn best_mv(&self, exp: f64, col: f64, limit: u128) -> ActionPath {
        let skills = SkillLevel::One(Strength::new(exp, col, Limit::Time(limit)));
        let mut mcts = MCTS::new(*self, self.turn, vec![Player::North, Player::South], skills);
        mcts.run()
    }
}

impl State<ActionPath, Player, ApiError> for Board {
    fn is_terminal(&self) -> bool {
        self.get_reward() != Reward::Continue
    }

    fn get_reward(&self) -> Reward<Player> {
        if self.north == 0 {
            return Reward::WonBy(Player::South);
        }

        if self.south == 0 {
            return Reward::WonBy(Player::North);
        }

        let Qmvs { north: n, south: s } = self.qmvs;
        if n >= 20 || s >= 20 {
            return Reward::Draw;
        }

        let possible_mvs = self.get_actions();

        if possible_mvs.len() == 0 {
            return Reward::WonBy(!self.turn);
        }

        Reward::Continue
    }

    fn apply_action(&self, action: &ActionPath) -> Result<(Self, Player), ApiError> {
        // let mut board = self.clone();
        let Some(state) = self.play(*action) else {
            return Err(ApiError::IllegalMove);
        };

        Ok((state, state.turn))
    }

    fn get_current_player(&self) -> &Player {
        &self.turn
    }

    fn view(&self) -> String {
        self.to_string()
    }

    fn get_actions(&self) -> Vec<ActionPath> {
        self.options(self.turn)
    }
}

impl Index<Player> for Board {
    type Output = u32;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::North => &self.north,
            Player::South => &self.south,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = 8;
        writeln!(f, "---------------------------------------------------")?;

        for row in (0..8).rev() {
            write!(f, "{} ", row + 1)?;

            for col in 0..8 {
                let is_dark = (row + col) % 2 == 0;
                if col == 0 {
                    write!(f, "|")?;
                }

                if is_dark {
                    let cell_in64bits = (row * rows) + col;
                    let cell_in32bits = cell_in64bits / 2;

                    let cell = 1 << cell_in32bits;

                    // let cell = 1u32 << bit_index;

                    let is_king = (self.kings & cell) != 0;
                    let is_south = (self.south & cell) != 0;
                    let is_north = (self.north & cell) != 0;

                    let piece = match (is_south, is_north, is_king) {
                        (true, false, false) => "B",
                        (true, false, true) => "BK",
                        (false, true, false) => "W",
                        (false, true, true) => "WK",
                        _ => "",
                    };

                    write!(f, " {:^3} |", piece)?;
                } else {
                    write!(f, " {:^3} |", "")?;
                }
            }

            writeln!(f)?;
            writeln!(f, "---------------------------------------------------")?;
        }

        writeln!(f, "  |  A  |  B  |  C  |  D  |  E  |  F  |  G  |  H  | ")?;
        writeln!(f, "---------------------------------------------------")?;

        writeln!(f, "Turn: {:?}", self.turn)?;
        writeln!(f, "Quiet moves: {:?}", self.qmvs)?;
        writeln!(f, "South: {:08x}", self.south)?;
        writeln!(f, "North: {:08x}", self.north)?;
        writeln!(f, "Kings: {:08x}", self.kings)?;
        Ok(())
    }
}
