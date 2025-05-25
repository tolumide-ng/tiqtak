use core::f64;
use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::mcts::{
    traits::{Action, MCTSError, Player},
    utils::{limit::Limit, rand::genrand, skill_level::SkillLevel},
};

use super::{node::Node, state::State};

pub struct MCTS<S, A, P, E>
where
    S: State<A, P, E>,
    A: Action,
    P: Player,
    E: MCTSError,
{
    root_node: Rc<RefCell<Node<S, A, P, E>>>,
    // The provided skill level for MCTS, this determines how indepth it would search, and how good the result would be
    level: SkillLevel,
    whoami: P,
    players: Vec<P>,
}

impl<S, A, P, E> MCTS<S, A, P, E>
where
    S: State<A, P, E>,
    A: Action,
    P: Player,
    E: MCTSError,
{
    pub fn new(state: S, turn: P, players: Vec<P>, level: SkillLevel) -> Self {
        let root_node = Node::new(None, state, None, turn, players.clone());

        Self {
            root_node,
            level,
            whoami: turn,
            players,
        }
    }

    /// use this when the node has a parent
    pub fn with(node: Rc<RefCell<Node<S, A, P, E>>>, players: Vec<P>, level: SkillLevel) -> Self {
        let whoami = *node.as_ref().borrow().get_current_player();

        Self {
            root_node: node,
            level,
            whoami,
            players,
        }
    }

    /// Choose a random action. Heuristics can be used to improved simulations
    pub fn choose(&self, actions: Vec<A>) -> A {
        if actions.len() == 1 {
            return actions[0];
        }

        let index = genrand(0, actions.len());

        actions[index]
    }

    /// Sim,ulate until a terminal state
    /// Player: Represent the player we care about to win (the compyter/bot)
    fn simulate(&self, node: Rc<RefCell<Node<S, A, P, E>>>) -> Vec<(&P, f64)> {
        let mut current_node = node;
        let mut local_stats = self.players.iter().map(|p| (p, 0f64)).collect::<Vec<_>>();

        let loss_penalty = self.level.loss_penalty();

        while !current_node.as_ref().borrow().is_terminal() {
            let action = self.choose(current_node.borrow().get_actions());

            let (next_state, next_player) = current_node.as_ref().borrow().execute(&action);

            let new_child = Node::new(
                Some(Rc::downgrade(&current_node.as_ref().borrow().me())),
                next_state,
                Some(action),
                next_player,
                self.players.clone(),
            );

            // Cumulate the reward
            if new_child.as_ref().borrow().is_terminal() {
                // we're unwrapping because we know that we're in a terminal state, hence, tbis would always return a `Some``
                for p in &self.players {
                    let reward = new_child.borrow().get_reward_for(p, loss_penalty).unwrap();

                    local_stats
                        .iter_mut()
                        .find(|(pl, _)| *pl == p)
                        .and_then(|(_, s)| Some(*s += reward))
                        .unwrap();
                }
            }

            current_node = new_child
        }

        local_stats
    }

    pub fn run(&mut self) -> A {
        let start = Instant::now();

        let node = self.root_node.clone();
        let constant = self.level.exploration_constant();

        match self.level.limit() {
            Limit::Time(time_limit) => {
                while &start.elapsed().as_millis() <= time_limit {
                    let selected_node = node.as_ref().borrow_mut().select(constant);
                    if !selected_node.as_ref().borrow().is_terminal() {
                        let child = selected_node.as_ref().borrow_mut().expand();
                        let rewards = self.simulate(child);
                        selected_node.as_ref().borrow_mut().back_propagate(rewards);
                    }
                }
            }
            Limit::Iterations(_count) => {}
        }

        self.optimize_for_win()
    }

    /// Returns the move with the max reward
    pub fn get_action_with_max_reward(&self) -> A {
        let root_node = self.root_node.borrow();
        let children = root_node.get_children();

        let mut max_reward = f64::NEG_INFINITY;
        let mut best_children = vec![];

        for child in children.iter() {
            let stats = &child.borrow().stats;
            let (_, reward) = stats.iter().find(|(p, _)| *p == self.whoami).unwrap();

            if *reward > max_reward {
                max_reward = *reward;
                best_children = vec![Rc::clone(child)];
            } else if *reward == max_reward {
                best_children.push(Rc::clone(child));
            }
        }

        let index = genrand(0, best_children.len());
        let child = &best_children[index];

        child.borrow().get_action().unwrap()
    }

    /// Gets a very competitive player that blocks the other player's succcess
    /// while ensuring that itself wins and makes this move if this move is a terminal
    /// meaning that it would ignore a possible opponent's win in the next round,
    /// if itself making this move means that it would win the game
    fn optimize_for_win(&self) -> A {
        let root_node = self.root_node.borrow();
        let children = root_node.get_children();

        let mut max_reward = f64::NEG_INFINITY;
        let mut best_children = vec![];
        let mut winning_moves = vec![];

        for child in children.iter() {
            let stats = &child.borrow().stats;
            let (_, reward) = stats.iter().find(|(p, _)| *p == self.whoami).unwrap();

            if child.borrow().is_terminal() {
                winning_moves.push(Rc::clone(child));
            } else if *reward > max_reward {
                max_reward = *reward;
                best_children = vec![Rc::clone(child)]
            } else if *reward == max_reward {
                best_children.push(Rc::clone(child));
            }
        }

        let mut child = None;

        if winning_moves.len() > 0 {
            child = Some(&winning_moves[genrand(0, winning_moves.len())]);
        } else {
            child = Some(&best_children[genrand(0, best_children.len())]);
        }

        child.unwrap().borrow().get_action().unwrap()
    }
}
