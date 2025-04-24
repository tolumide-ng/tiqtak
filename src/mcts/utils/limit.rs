/// Thinking time for mcts
#[derive(Debug, Clone, Copy)]
pub enum Limit {
    Time(u128), // in ms
    Iterations(u64),
}
