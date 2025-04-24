use std::{error::Error, fmt::Debug, hash::Hash};

/// The game action (e.g move to)
pub(crate) trait Action: Debug + Eq + PartialEq + Hash + Clone + Copy {}

/// Basic errors from this algo
pub(crate) trait MCTSError: Error {}

/// The player(s) who would be playing this game
pub(crate) trait Player: Debug + PartialEq + Eq + Hash + Copy + Clone {}

// / Player and the player's stats (player, player_stats_data)
// pub(crate) struct Players<T: Player>(Vec<(T, f64)>);

// impl<T> From<Vec<T>> for Players<T>
// where
//     T: Player,
// {
//     fn from(value: Vec<T>) -> Self {
//         Self(value.into_iter().map(|p| (p, 0f64)).collect::<Vec<_>>())
//     }
// }

// impl<T> Deref for Players<T>
// where
//     T: Player,
// {
//     type Target = Vec<(T, f64)>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T: Player> DerefMut for Players<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
