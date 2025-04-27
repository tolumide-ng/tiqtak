use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
};

/// The game action (e.g move to)
pub(crate) trait Action: Debug + Eq + PartialEq + Clone + Copy + Display {}

/// Basic errors from this algo
pub(crate) trait MCTSError: Error {}

/// The player(s) who would be playing this game
pub(crate) trait Player: Debug + PartialEq + Eq + Hash + Copy + Clone {}
