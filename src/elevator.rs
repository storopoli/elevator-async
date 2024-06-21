use std::sync::{Arc, Mutex};

use crate::utils::Direction;

/// An elevator that can move up and down between floors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Elevator {
    /// Current floor.
    pub(crate) current_floor: u8,
    /// An optional [`Direction`] that the elevator is moving in
    /// or `None` if the elevator is stopped.
    pub(crate) direction: Option<Direction>,
    /// Destination floor(s) while moving [`Direction::Up`].
    pub(crate) up_queue: Vec<u8>,
    /// Destination floor(s) while moving [`Direction::Down`].
    pub(crate) down_queue: Vec<u8>,
}

impl Elevator {
    /// Create a new [`Elevator`] with an optional current floor.
    pub(crate) fn new(current_floor: Option<u8>) -> Self {
        Self {
            current_floor: current_floor.unwrap_or(1),
            direction: None,
            // Let's say we have 12 floors for now
            up_queue: Vec::with_capacity(12),
            down_queue: Vec::with_capacity(12),
        }
    }

    /// Moves the [`Elevator`] according to the current direction
    /// and the destination queue.
    ///
    /// # Note
    ///
    /// The function should be called `move` but it's a reserved keyword in Rust.
    pub(crate) fn tick(&mut self) {
        if let Some(dir) = self.direction {
            if self.up_queue.is_empty() && self.down_queue.is_empty() {
                // remove the direction if there are no more destinations
                self.direction = None;
            }

            match dir {
                Direction::Up => {
                    if let Some(dest) = self.up_queue.first() {
                        match dest.cmp(&self.current_floor) {
                            std::cmp::Ordering::Greater => {
                                self.current_floor += 1;
                                println!("Elevator moving up to floor {}", self.current_floor);
                            }
                            std::cmp::Ordering::Equal => {
                                self.up_queue.remove(0);
                                println!("Elevator stopped at floor {}", self.current_floor);
                            }
                            std::cmp::Ordering::Less => {
                                self.direction = None;
                            }
                        }
                    }
                }
                Direction::Down => {
                    if let Some(dest) = self.down_queue.first() {
                        match dest.cmp(&self.current_floor) {
                            std::cmp::Ordering::Less => {
                                self.current_floor -= 1;
                                println!("Elevator moving down to floor {}", self.current_floor);
                            }
                            std::cmp::Ordering::Equal => {
                                self.down_queue.remove(0);
                                println!("Elevator stopped at floor {}", self.current_floor);
                            }
                            std::cmp::Ordering::Greater => {
                                self.direction = None;
                            }
                        }
                    }
                }
            }
        } else {
            // if we don't have a direction, we should check the queues
            if let Some(dest) = self.up_queue.first() {
                if dest > &self.current_floor {
                    self.direction = Some(Direction::Up);
                }
            } else if let Some(dest) = self.down_queue.first() {
                if dest < &self.current_floor {
                    self.direction = Some(Direction::Down);
                }
            }
        }
    }

    /// Add a destination floor to the elevator's queue.
    pub(crate) fn add_destination(&mut self, floor: u8, direction: Direction) {
        match direction {
            Direction::Up => self.up_queue.push(floor),
            Direction::Down => self.down_queue.push(floor),
        }
        match direction {
            Direction::Up => self.up_queue.sort_unstable(),
            Direction::Down => self.down_queue.sort_unstable(),
        }
        if self.direction.is_none() {
            self.direction = Some(direction);
        }
    }
}

pub(crate) async fn move_elevator(elevator_arc: Arc<Mutex<Elevator>>) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let mut elevator = elevator_arc.lock().expect("Mutex poisoned");
        elevator.tick();

        #[cfg(debug_assertions)]
        println!("Elevator state: {elevator:?}");

        drop(elevator); // release the guard
    }
}
