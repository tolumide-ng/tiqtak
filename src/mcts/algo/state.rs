use std::fmt::{Debug, Display};

use crate::mcts::{
    traits::{Action, MCTSError, Player},
    utils::reward::Reward,
};

/// Priority based on Move importance: Prioritize expansion of nodes corresponding to moves that are
pub trait State<A, P, E>: Clone + Debug + Display
where
    A: Action,
    P: Player,
    E: MCTSError,
{
    /// Returns true if the game is a draw, or won by one of the players
    /// Otherwise, returns false
    fn is_terminal(&self) -> bool;

    /// Returns Option<Player> that won the game
    /// returns None, if no one won
    fn get_reward(&self) -> Reward<P>;

    /// Duplicate the action before applying it to self
    /// return the new state that was initially diplicated **and the applied action
    /// with the next player
    /// IMPORTANT: This would alter the state of the board (todo: would be fixed later!!!)
    /// Hence, the state needs to be duplicated first before the action is applied to the state
    /// Returns the new State of and the next player (that is the only player that can play on this new state)
    fn apply_action(&self, action: &A) -> Result<(Self, P), E>;

    /// Returns all the possible moves for the current state of the "BOARD" (e.g get_moves)
    fn get_actions(&self) -> Vec<A>;

    fn get_current_player(&self) -> &P;

    fn view(&self) -> String;
}
