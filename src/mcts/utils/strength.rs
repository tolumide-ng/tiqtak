use super::limit::Limit;

#[derive(Debug, Clone, Copy)]
pub struct Strength {
    /// Exploration constant of the MCTS formula
    pub(crate) e: f64,
    /// The cost of losing the game in the MCTS approach (simulation function)
    pub(crate) cost: f64,
    /// The amount of time MCTS should spend exploring + exploiting its options
    pub(crate) limit: Limit,
}

impl Strength {
    pub fn new(e: f64, cost: f64, limit: Limit) -> Self {
        Self { e, cost, limit }
    }
}
