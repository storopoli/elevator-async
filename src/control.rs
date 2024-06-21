use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

use crate::{elevator::Elevator, message::Message, utils::Direction};

/// The elevator control system that manages the elevator's movement.
pub(crate) async fn elevator_control_system(
    mut rx: mpsc::Receiver<Message>,
    elevator_arc: Arc<Mutex<Elevator>>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::FloorSwitch(fs) => {
                let mut elevator = elevator_arc.lock().expect("Mutex poisoned");
                if fs.floor == elevator.current_floor {
                    elevator.direction = Some(fs.direction);
                }
                elevator.add_destination(fs.floor, fs.direction);
            }
            Message::ElevatorSwitch(es) => {
                let mut elevator = elevator_arc.lock().expect("Mutex poisoned");
                if es.destination > elevator.current_floor {
                    elevator.add_destination(es.destination, Direction::Up);
                } else {
                    elevator.add_destination(es.destination, Direction::Down);
                }
            }
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
