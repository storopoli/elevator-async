use serde::{Deserialize, Serialize};

/// Direction of the elevator
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) enum Direction {
    /// Going up
    Up,
    /// Going down
    Down,
}
