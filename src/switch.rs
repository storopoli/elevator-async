use serde::{Deserialize, Serialize};

use crate::utils::Direction;

/// Switch that the user activate on the floor
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct FloorSwitch {
    /// Current floor
    pub(crate) floor: u8,
    /// Direction chosen, either [`Direction::Up`]
    /// or [`Direction::Down`]
    pub(crate) direction: Direction,
}

/// Switch that the user activate inside the elevator
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct ElevatorSwitch {
    /// Destination floor
    pub(crate) destination: u8,
}
