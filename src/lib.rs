//! TiqTak is a fast, modern, and portable checkers engine, built with Rust and exposed to the web
//! via WebAssembly.
//! At its core, this engine leverage Monte Carlo Tree Search (MCTS) to provide adapative, probabilistic
//! decision-making that balance exploration and exploitation-making it well suited for both casual play,
//! and competitive AI development.
//! In the future, I plan to make the search heuristic used more dynamic such that it canbe provide  
//! by the user, rather than been forced to use the choices made by this author (me).
//! To create a new board
//! ```rust
//! use tiqtak::game::board::state::Board;
//!
//! let mut board = Board::new(); // creates a new board
//! println!("{board}");
//! let possible_mvs = board.options(board.turn); // returns a list of possible mvs for the player
//! let mv = board.best_mv(1.41421356237_f64, -1.25_f64, 100); // only use this for a bot
//! let is_valid = board.is_valid(mv, board.turn);
//! assert!(possible_mvs.contains(&mv));
//! assert!(is_valid);
//! let new_board = board.play(mv).unwrap(); // return None if the mv is invalid
//! println!("{board}");
//! assert_ne!(board, new_board);
//! ```

pub mod game;
pub(crate) mod mcts;
