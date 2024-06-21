/// Direction of the elevator
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Direction {
    /// Going up
    Up,
    /// Going down
    Down,
}
