#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Index;
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

use super::scale::Scale;

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(feature = "history"), derive(Copy))]
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
    #[cfg(feature = "history")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) prev: Vec<Self>,
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
            #[cfg(feature = "history")]
            prev: Vec::new(),
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
    pub fn with(
        north: u32,
        south: u32,
        kings: u32,
        turn: Player,
        qmvs: Qmvs,
        #[cfg(feature = "history")] prev: Vec<Self>,
    ) -> Self {
        Self {
            north,
            south,
            kings,
            turn,
            qmvs,
            #[cfg(feature = "history")]
            prev,
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
    pub fn is_valid(&self, path: ActionPath, turn: Player) -> bool {
        assert!(
            path.len > 0,
            "Invalid Action: There must be atleast one move in an action"
        );

        let mut action = Action::from(path[0]);
        if action.scale == Scale::U64 {
            action = action.transcode();
        }

        let src_mask = 1u32 << action.src;

        if (self[turn] & src_mask) == 0 {
            return false;
        }

        let result = BitBoard::new(1 << action.src, self[!turn], self[turn], self.kings).get(turn);

        println!("{:?}", result);

        if path.scale == Scale::U64 {
            return result.contains(&path.transcode());
        } else {
            return result.contains(&path);
        }
    }

    /// Returns all the possible options(moves) that the selected user can play
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn options(&self, turn: Player) -> Vec<ActionPath> {
        let regulars = self.regular(turn);
        let kings = self.kings(turn);
        let opponent = self[!turn];

        BitBoard::new(regulars | kings, opponent, 0, self.kings).get(turn)
    }

    /// This returns a new Board state (the new board state) after the move (ActionPath) is applied to the board
    /// Please always provide only u64 format of the action for valid plays
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn play(&self, action: ActionPath) -> Option<Self> {
        if !self.is_valid(action, self.turn) {
            println!("that's a bad move");
            return None;
        }

        #[cfg(feature = "history")]
        let mut board = self.clone();
        #[cfg(not(feature = "history"))]
        let mut board = *self;

        for mv in &action.mvs[..action.len] {
            let mut action = Action::from(*mv);

            if action.scale == Scale::U64 {
                action = action.transcode();
            }

            let Action {
                src,
                tgt,
                capture,
                promoted,
                ..
            } = action;

            let src_mask = 1 << src;
            let tgt_mask = 1 << tgt;

            let captured = match capture {
                true => BitBoard::new(src_mask, self[!self.turn], self[self.turn], self.kings)
                    .captured(src, tgt),
                false => 0,
            };

            let is_king = (self.kings & src_mask) != 0;
            let kings = ((self.kings & !(1 << src)) & !(1 << captured))
                | (u32::from(is_king || promoted) << tgt);

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

            #[cfg(feature = "history")]
            let prev = {
                let mut history = std::mem::take(&mut board.prev);
                history.push(board);
                history
            };

            board = Self::with(
                north,
                south,
                kings,
                turn,
                qmvs,
                #[cfg(feature = "history")]
                prev,
            );
        }

        board.turn = !self.turn;
        return Some(board);
    }

    #[cfg_attr(all(feature = "web", feature = "serde"), wasm_bindgen)]
    #[cfg(feature = "history")]
    pub fn logs(&self) -> js_sys::Array {
        self.prev
            .iter()
            .map(|b| serde_wasm_bindgen::to_value(b).unwrap())
            .collect()
    }

    #[cfg_attr(feature = "web", wasm_bindgen)]
    #[cfg(feature = "history")]
    pub fn log_count(&self) -> usize {
        self.prev.len()
    }

    #[cfg_attr(feature = "web", wasm_bindgen)]
    #[cfg(feature = "history")]
    pub fn undo(&mut self, player: Player) -> Result<Self, ApiError> {
        let mut logs = std::mem::take(&mut self.prev);
        let has_enough_history = logs.len() >= 2;
        // we can only undo in this case if this player has played before, else abort the game themselve
        if !has_enough_history {
            return Err(ApiError::IllegalMove);
        }

        if player == self.turn {
            logs.pop();
        }

        let mut board = logs.pop().unwrap();
        board.prev = logs;

        return Ok(board);
    }

    /// Generates the next best move based on the provided MCTS configuration
    /// NB: Only use this method when you're trying to get a bot's next move  
    /// exp: exploration constant for MCTS
    /// col: cost of losing (recommended ==> -1.25)
    /// limit: How long should MCTS think (in ms)? (recommended 100)
    #[cfg_attr(feature = "web", wasm_bindgen)]
    pub fn best_mv(&self, exp: f64, col: f64, limit: u128) -> ActionPath {
        let skills = SkillLevel::One(Strength::new(exp, col, Limit::Time(limit)));
        #[cfg(not(feature = "history"))]
        let state = *self;
        #[cfg(feature = "history")]
        let state = self.clone();
        let mut mcts = MCTS::new(state, self.turn, vec![Player::North, Player::South], skills);
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
        match self.play(*action) {
            Some(state) => {
                let turn = state.turn;
                return Ok((state, turn));
            }
            _ => Err(ApiError::IllegalMove),
        }
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "history")]
    #[cfg(test)]
    mod undo_moves {
        use crate::{
            Action, Board,
            Scale::*,
            game::{model::player::Player, utils::ApiError},
        };

        #[test]
        fn should_undo_two_moves_when_it_is_players_turn() {
            let mut board = Board::new();
            assert_eq!(board.prev.len(), 0);

            let mv_0 = Action::new(9, 13, false, false, U32).into();
            let mv_1 = Action::new(23, 19, false, false, U32).into();
            let mv_2 = Action::new(11, 14, false, false, U32).into();

            board = board.play(mv_0).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xfff00000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 1);

            board = board.play(mv_1).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xff780000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 2);

            board = board.play(mv_2).unwrap();
            assert_eq!(board.south, 0x000065ff);
            assert_eq!(board.north, 0xff780000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 3);
            assert_eq!(board.prev[0].prev.len(), 0);
            assert_eq!(board.prev[1].prev.len(), 0);
            assert_eq!(board.prev[2].prev.len(), 0);

            board = board.undo(Player::North).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xfff00000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.turn, Player::North);
            assert_eq!(board.prev.len(), 1);
        }

        #[test]
        fn should_undo_one_move_if_its_not_the_users_turn() {
            let mut board = Board::new();
            assert_eq!(board.prev.len(), 0);

            let mv_0 = Action::new(9, 13, false, false, U32).into();
            let mv_1 = Action::new(23, 19, false, false, U32).into();
            let mv_2 = Action::new(11, 14, false, false, U32).into();

            board = board.play(mv_0).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xfff00000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 1);

            board = board.play(mv_1).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xff780000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 2);

            board = board.play(mv_2).unwrap();
            assert_eq!(board.south, 0x000065ff);
            assert_eq!(board.north, 0xff780000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 3);
            assert_eq!(board.prev[0].prev.len(), 0);
            assert_eq!(board.prev[1].prev.len(), 0);
            assert_eq!(board.prev[2].prev.len(), 0);

            assert_eq!(board.turn, Player::North);
            board = board.undo(Player::South).unwrap();
            assert_eq!(board.prev.len(), 2);
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xff780000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.turn, Player::South);
        }

        #[test]
        fn undo_rejected_if_player_turn_and_game_start() {
            let mut board = Board::new();
            assert_eq!(board.prev.len(), 0);

            let result = board.undo(Player::South);
            assert_eq!(result, Err(ApiError::IllegalMove));
        }

        #[test]
        fn undo_rejected_if_not_player_turn_but_game_start() {
            let mut board = Board::new();
            assert_eq!(board.prev.len(), 0);

            let result = board.undo(Player::North);
            assert_eq!(result, Err(ApiError::IllegalMove));
        }

        #[test]
        fn undo_rejected_if_not_enough_games() {
            let mut board = Board::new();
            assert_eq!(board.prev.len(), 0);

            let mv_0 = Action::new(9, 13, false, false, U32).into();

            board = board.play(mv_0).unwrap();
            assert_eq!(board.south, 0x00002dff);
            assert_eq!(board.north, 0xfff00000);
            assert_eq!(board.kings, 0);
            assert_eq!(board.prev.len(), 1);

            let result = board.undo(Player::North);
            assert_eq!(result, Err(ApiError::IllegalMove));

            let result = board.undo(Player::South);
            assert_eq!(result, Err(ApiError::IllegalMove));
        }
    }
}
