use game::{board::Board, utils::Player};
use mcts::utils::{limit::Limit, skill_level::SkillLevel, strength::Strength};

pub(crate) mod game;
pub(crate) mod mcts; // should this be moved into a separate workspace?

fn main() {
    for i in 0..4 {
        let north = Player::North;
        let south = Player::South;
        let players = vec![north, south];

        let exploration_constant = 1.41421356237_f64;
        let cost_of_losing = -1.25_f64;
        let limit = Limit::Time(1000);
        let skills = SkillLevel::Two(Strength::new(exploration_constant, cost_of_losing, limit));

        let board = Board::new();
    }

    // println!("{board}");
}
