use crate::mcts::{algo::state::State, utils::reward::Reward};

use super::{
    board::checkers::Board,
    path::ActionPath,
    utils::{AppError, Player},
};

impl State<ActionPath, Player, AppError> for Board {
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

        let (n, s) = self.qmvs;
        if n == 20 || s == 20 {
            return Reward::Draw;
        }

        let possible_mvs = self.get_actions();

        if possible_mvs.len() == 0 {
            return Reward::WonBy(!self.turn);
        }

        Reward::Continue
    }

    fn apply_action(&self, action: &ActionPath) -> Result<(Self, Player), AppError> {
        // let mut board = self.clone();
        let Some(state) = self.play(*action) else {
            return Err(AppError::IllegalMove);
        };

        Ok((state, state.turn))
    }

    fn get_current_player(&self) -> &Player {
        &self.turn
    }

    fn view(&self) -> String {
        self.to_string()
    }

    /// WHY AM I HANDLING THIS HERE: preamble: I just wasted one hour trying to debug something that isn't even an issue in the first place 🤦🏾‍♂️
    /// Why????
    /// The options generation would always return all the possible moves for the current state of the board
    /// for e.g if a move include A -> B -> C -> D (which means we should have [(a, b), (b, c), (c, d)]
    /// the dumb mcts I wrote here doesn't know this, and would randomly select any of the moves, without understanding that there should be an order
    /// which would result in an invalid game because there is probably no piece beloning to this user at (target b), that would be moved to c in the first place.
    /// So, what this does is that it ensures only the originating moves of jumps moves are provided as possible actions in the first place
    /// NOTE: THIS IS STUPID AND NEEDS TO BE UPDATED;
    /// WHY???? COMPUTER CANNOT CURRENTLY PLAY JUMPING MOVES, WE NEED IT TO BE ABLE TO DO THAT!!
    fn get_actions(&self) -> Vec<ActionPath> {
        self.options(self.turn)
    }
}
