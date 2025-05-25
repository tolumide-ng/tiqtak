#[derive(Debug, PartialEq, Eq)]
pub enum Reward<P> {
    Draw,
    Continue,
    WonBy(P),
}
