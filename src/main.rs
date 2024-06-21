use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::channel;

pub mod control;
pub mod elevator;
pub mod message;
pub mod request;
pub mod switch;
pub mod utils;

use control::{elevator_control_system, move_elevator};
use elevator::Elevator;
use request::handle_request;

#[tokio::main]
async fn main() {
    let elevator = Elevator::new(None);
    let elevator_arc = Arc::new(Mutex::new(elevator));

    let (tx, rx) = channel(10);

    // spawn the elevator control system task
    tokio::spawn(elevator_control_system(rx, elevator_arc.clone()));

    // spawn the elevator movement task
    tokio::spawn(move_elevator(elevator_arc.clone()));

    // create an API server to receive requests
    let server = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    loop {
        let (socket, _) = server.accept().await.expect("Failed to accept connection");

        tokio::spawn(handle_request(tx.clone(), socket));
    }
}
