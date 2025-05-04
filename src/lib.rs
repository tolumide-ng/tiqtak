//! TiqTak is a fast, modern, and portable checkers engine, built with Rust and exposed to the web
//! via WebAssembly.
//! At its core, this engine leverage Monte Carlo Tree Search (MCTS) to provide adapative, probabilistic
//! decision-making that balance exploration and exploitation-making it well suited for both casual play,
//! and competitive AI development.
//! In the future, I plan to make the search heuristic used more dynamic such that it canbe provide  
//! by the user, rather than been forced to use the choices made by this author (me).

pub mod game;
pub(crate) mod mcts;
