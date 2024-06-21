use serde::{Deserialize, Serialize};

use crate::switch::{ElevatorSwitch, FloorSwitch};

/// Message enum.
///
/// This enum is used to send messages between the elevator and the floors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) enum Message {
    /// Switch that the user activate on the floor.
    FloorSwitch(FloorSwitch),
    /// Switch that the user activate inside the elevator.
    ElevatorSwitch(ElevatorSwitch),
}
