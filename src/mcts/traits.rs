use std::{error::Error, fmt::Debug, hash::Hash};

/// The player(s) who would be playing this game
pub(crate) trait Player: Debug + Hash + PartialEq + Eq {}

/// The game action (e.g move to)
pub(crate) trait Action: Debug + Eq + PartialEq + Hash {}

/// Basic errors from this algo
pub(crate) trait MCTSError: Error {}
