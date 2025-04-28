use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::mcts::{
    traits::{Action, MCTSError, Player},
    utils::{
        rand::{genrand, getrand},
        reward::Reward,
    },
};

use super::{bandit::MultiArmedBandit, state::State};

type Parent<S, A, P, E> = Option<Weak<RefCell<Node<S, A, P, E>>>>;
pub struct Node<S: State<A, P, E>, A: Action, P: Player, E: MCTSError> {
    parent: Parent<S, A, P, E>,
    pub(crate) visits: f64,
    pub(crate) children: Vec<Rc<RefCell<Node<S, A, P, E>>>>,
    pub state: S,
    /// the move or action generated for this node
    action: Option<A>,
    /// statistics of the wins on the node for each player
    pub(crate) stats: Vec<(P, f64)>,
    pub(crate) player: P,
    me: Weak<RefCell<Node<S, A, P, E>>>,
}

impl<S, A, P, E> MultiArmedBandit for Node<S, A, P, E>
where
    S: State<A, P, E>,
    A: Action,
    P: Player,
    E: MCTSError,
{
}

impl<S, A, P, E> Node<S, A, P, E>
where
    S: State<A, P, E>,
    A: Action,
    P: Player,
    E: MCTSError,
{
    pub(crate) fn new(
        parent: Parent<S, A, P, E>,
        state: S,
        action: Option<A>,
        player: P,
        players: Vec<P>,
    ) -> Rc<RefCell<Self>> {
        let stats = players.into_iter().map(|p| (p, 0_f64)).collect::<Vec<_>>();
        Rc::new_cyclic(|me| {
            RefCell::new(Self {
                parent,
                visits: 0f64,
                children: vec![],
                state,
                action,
                stats,
                player,
                me: me.clone(),
            })
        })
    }

    pub fn my_stats(&self) -> f64 {
        self.stats
            .iter()
            .find(|(p, _)| *p == self.player)
            .unwrap()
            .1
    }

    pub fn stats(&self) -> Vec<f64> {
        self.stats.iter().map(|(_, s)| *s).collect()
    }

    /// this is only valuable when this node is a terminal
    /// If the game is terminal (ended)
    /// It returns the person that won the game
    /// or returns Draw if the game is a draw
    /// It returns `Reward::Continue` if the node is not a terminal
    pub fn get_reward(&self) -> Reward<P> {
        self.state.get_reward()
    }

    /// Gets the reward for the provided player
    /// If this node is a terminal node (game is completed),
    /// then it returns a `Some` else it returns `None`
    /// In the case where the game is completed, it returns the following:
    /// Some(1) - if the provided player won the game
    /// ~~Some(0) - if the provided player lost the game~~
    /// Some(-0.1) - if the provided player lost the game
    /// Some(0.5) - if the game was a draw
    /// CONSIDER SHIFTING THE IMPLEMENTATION OF THIS TO THE CLIENT LIBRARY
    pub fn get_reward_for(&self, player: &P, cost_of_losing: f64) -> Option<f64> {
        let reward = self.state.get_reward();
        match reward {
            Reward::WonBy(winning_player) => {
                if &winning_player == player {
                    return Some(1_f64);
                }
                // Change this value for a loss depending on how complex you want the bot to be
                // -1.25 (Hard) - Punishes itself hard for allowing the opponent to win (reduces -1.25 for such path)
                // -1.0 (Medium) - Removes the equivalent of a win from its own point if the opponent wins
                // -0.90 (Easy) - Okay with the opponent winning but, but still interested in winning
                // return Some(-1_f64);
                return Some(cost_of_losing);
            }
            Reward::Draw => Some(0.5_f64),
            Reward::Continue => None,
        }
    }

    pub fn get_children(&self) -> &Vec<Rc<RefCell<Node<S, A, P, E>>>> {
        return &self.children;
    }

    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    pub fn get_action(&self) -> &Option<A> {
        &self.action
    }

    pub fn get_actions(&self) -> Vec<A> {
        self.state.get_actions()
    }

    fn get_unexpanded_actions(&self) -> Vec<A> {
        let actions = self.get_actions();

        let expanded_children = self
            .children
            .iter()
            .filter_map(|c| c.as_ref().borrow().action)
            .collect::<Vec<_>>();

        // println!(
        //     "the total obtained actions here is >>>>>|||||||| {}, and the expanded_children are ------>>>> ((((((({})))))",
        //     actions.len(),
        //     expanded_children.len()
        // );
        // println!(
        //     "the expanded children are {:?}, \n\n actions are: {:?}",
        //     expanded_children, actions
        // );
        return actions
            .into_iter()
            .filter(|a| !expanded_children.contains(a))
            .collect::<Vec<_>>();
    }

    fn is_fully_expanded(&self) -> bool {
        self.state.get_actions().len() == self.children.len()
    }

    pub fn me(&self) -> Rc<RefCell<Self>> {
        self.me.upgrade().unwrap()
    }

    pub fn select(&self, constant: f64) -> Rc<RefCell<Node<S, A, P, E>>> {
        if !self.is_fully_expanded() || self.is_terminal() {
            return Rc::clone(&self.me());
        }

        // Assuming this node is already fully expanded
        // (i.e. all it's children have been explored),
        // we need to make an informed decision about which of it's
        // children to select to become the next node under scope
        let result = Self::mdp_select(&self, constant);

        return Rc::clone(&result);
    }

    pub fn expand(&mut self) -> Rc<RefCell<Node<S, A, P, E>>> {
        let game_ended = self.children.len() == self.state.get_actions().len();

        let actions = self.get_unexpanded_actions();

        // println!(
        //     "\n\nthe total children is >>>> {}, and the total actions is {}, the unexpanded action len is {} \n\n\n\n",
        //     self.children.len(),
        //     self.state.get_actions().len(),
        //     actions.len()
        // );

        // println!("the children here are ***** {game_ended}");

        if !game_ended {
            // let actions = self.get_unexpanded_actions();

            // actions.iter().for_each(|x| println!("{:?}", x.to_string()));

            // println!("the unexpanded actions are ????? {:?}", actions);

            let index = genrand(0, actions.len());
            let action = &actions[index];

            let child = self.get_outcome_child(*action);
            return child;
        }

        return Rc::clone(&self.me());
    }

    pub fn view(&self) -> String {
        self.state.view()
    }

    pub fn back_propagate(&mut self, rewards: Vec<(&P, f64)>) {
        self.visits += 1f64;

        for (player, reward) in &rewards {
            let player_stat = self.stats.iter_mut().find(|(p, _)| p == *player);
            if let Some((_, s)) = player_stat {
                *s += *reward
            }
        }

        if let Some(parent) = self.parent.as_ref().and_then(|p| p.upgrade()) {
            parent.as_ref().borrow_mut().back_propagate(rewards);
        }
    }

    /// Applies the provided action to a duplicate of the state
    /// Returns a new state (with the action already applied)
    pub fn execute(&self, action: &A) -> (S, P) {
        let (next_state, next_player) = self.state.apply_action(action).unwrap();
        return (next_state, next_player);
    }

    fn get_outcome_child(&mut self, action: A) -> Rc<RefCell<Node<S, A, P, E>>> {
        let (next_state, next_player) = self.execute(&action);

        let new_child = Self::new(
            Some(self.me.clone()),
            next_state,
            Some(action),
            next_player,
            self.stats.iter().map(|(p, _)| *p).collect::<Vec<_>>(),
        );

        self.children.push(Rc::clone(&new_child));

        return new_child;
    }

    pub fn get_current_player(&self) -> &P {
        &self.state.get_current_player()
    }
}
