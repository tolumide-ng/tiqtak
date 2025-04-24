use crate::mcts::utils::limit::Limit;

use super::strength::Strength;

#[derive(Debug)]
pub enum SkillLevel {
    Zero(Strength),
    One(Strength),
    Two(Strength),
}

impl SkillLevel {
    pub fn exploration_constant(&self) -> f64 {
        match self {
            Self::Zero(Strength { e, .. }) => *e,
            Self::One(Strength { e, .. }) => *e,
            Self::Two(Strength { e, .. }) => *e,
        }
    }

    pub fn loss_penalty(&self) -> f64 {
        match self {
            Self::Zero(Strength { cost, .. }) => *cost,
            Self::One(Strength { cost, .. }) => *cost,
            Self::Two(Strength { cost, .. }) => *cost,
        }
    }

    pub fn limit(&self) -> &Limit {
        match self {
            Self::Zero(Strength { limit, .. }) => limit,
            Self::One(Strength { limit, .. }) => limit,
            Self::Two(Strength { limit, .. }) => limit,
        }
    }
}
