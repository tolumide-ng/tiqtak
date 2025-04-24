#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Reward<P> {
    Draw,
    Continue,
    WonBy(P),
}
