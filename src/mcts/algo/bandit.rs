use core::f64;
use std::{cell::RefCell, rc::Rc};

use crate::mcts::{
    traits::{Action, MCTSError, Player},
    utils::rand::genrand,
};

use super::{node::Node, state::State};

pub(crate) trait MultiArmedBandit {
    /// Given that this node is fully expanded i.e all the direct children of this node have been explored
    /// This method helps us calculate the best child of this node to exploit further
    /// Selects an action for the state from a list given a Q-function(???) (https://gibberblot.github.io/rl-notes/single-agent/multi-armed-bandits.html#id9)
    /// this can be: Softmax strategy, UCB1 e.t.c
    fn mdp_select<S: State<A, P, E>, A: Action, P: Player, E: MCTSError>(
        node: &Node<S, A, P, E>,
        constant: f64,
    ) -> Rc<RefCell<Node<S, A, P, E>>> {
        let mut max_actions: Vec<Rc<RefCell<Node<S, A, P, E>>>> = vec![];
        let mut max_value = f64::NEG_INFINITY;

        let game_is_just_startiong = node
            .children
            .iter()
            .find(|c| c.as_ref().borrow().visits == 0f64);

        for child in &node.children {
            let total_games_won = child.as_ref().borrow().my_stats();
            let total_visits = child.as_ref().borrow().visits;

            let total_visits = if let Some(_) = game_is_just_startiong {
                f64::INFINITY
            } else {
                total_visits
            };

            let result = (total_games_won / total_visits)
                + (constant * f64::sqrt(node.visits.ln() / total_visits));

            if result > max_value {
                max_value = result;
                max_actions = vec![Rc::clone(child)]
            } else if result == max_value {
                max_actions.push(Rc::clone(child));
            }
        }

        let index = genrand(0, max_actions.len());
        Rc::clone(&max_actions[index])
    }
}
