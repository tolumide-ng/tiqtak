use game::{board::state::Board, model::action::Action, model::player::Player};
use mcts::{
    algo::{state::State, tree_search::MCTS},
    utils::{limit::Limit, reward::Reward, skill_level::SkillLevel, strength::Strength},
};

pub(crate) mod game;
pub(crate) mod mcts; // should this be moved into a separate workspace?

fn main() {
    for i in 0..6 {
        println!("------------------ROUND {}------------------", i);
        let north = Player::North;
        let south = Player::South;
        let players = vec![north, south];

        let exploration_constant = 1.41421356237_f64;
        let cost_of_losing = -1.25_f64;
        // let limit = Limit::Time(1000);
        let limit = Limit::Time(100);
        let skills = SkillLevel::Two(Strength::new(exploration_constant, cost_of_losing, limit));

        let mut board = Board::new();

        while board.get_reward() == Reward::Continue {
            let mut mcts = MCTS::new(board, board.turn, players.clone(), skills);

            let mv = mcts.run();

            mv.iter().for_each(|x| println!("{} -->", Action::from(*x)));
            board = board.play(mv).unwrap();

            println!("{board}");
        }

        match board.get_reward() {
            Reward::WonBy(p) => {
                println!(
                    "--------------------------------------------PLAYER {:?} WON---------------------------------------------- \n\n\n\n",
                    p
                );
            }
            _ => {
                println!(
                    "--------------------------------------------DRAW---------------------------------------------- \n\n\n\n"
                );
            }
        }

        // while board.
    }

    // println!("{board}");
}
