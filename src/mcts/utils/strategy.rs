pub(crate) enum Strategy {
    HighestScore,
    /// optimizes for the child with highest score but chooses a terminal child (that wins) if there is one
    TerminalInclined,
    /// child with the most visits
    RoubstChild,
}
