use crate::{
    Board, Player,
    mcts::{algo::state::State, utils::reward::Reward},
};

use super::{action::Action, utils::AppError};

impl State<Action, Player, AppError> for Board {
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

        let possible_mvs = self.get_actions();

        if possible_mvs.len() == 0 {
            return Reward::WonBy(!self.turn);
        }

        Reward::Continue
    }

    fn apply_action(&self, action: &Action) -> Result<(Self, Player), AppError> {
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

    fn get_actions(&self) -> Vec<Action> {
        self.options(self.turn)
    }
}
