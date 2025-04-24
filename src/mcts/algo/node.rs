use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::mcts::traits::{Action, MCTSError, Player};

use super::state::State;

type Parent<S, A, P, E> = Option<Weak<RefCell<Node<S, A, P, E>>>>;
pub struct Node<S: State<A, P, E>, A: Action, P: Player, E: MCTSError> {
    parent: Parent<S, A, P, E>,
    visits: f64,
    children: Vec<Rc<RefCell<Node<S, A, P, E>>>>,
    pub state: S,
    /// the move or action generated for this node
    actoin: Option<A>,
    /// statistics of the wins on the node for each player
    stats: HashMap<P, f64>,
    player: P,
    me: Weak<RefCell<Node<S, A, P, E>>>,
    players: Vec<P>,
}

impl<S, A, P, E> Node<S, A, P, E>
where
    S: State<A, P, E>,
    A: Action,
    P: Player,
    E: MCTSError,
{
}
