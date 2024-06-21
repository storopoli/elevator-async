use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

use crate::message::Message;

/// Handles incoming requests from the client
/// and sends them to the elevator control system
/// using the provided channel.
pub(crate) async fn handle_request(tx: Sender<Message>, socket: TcpStream) {
    let mut buf = [0; 1024];
    let mut stream = BufReader::new(socket);
    let n = stream
        .read(&mut buf)
        .await
        .expect("Failed to read data from socket");
    let request = String::from_utf8_lossy(&buf[..n]);

    #[cfg(debug_assertions)]
    println!("Received request: {request}");

    let input: Message = serde_json::from_str(&request).expect("Failed to parse JSON message");

    match input {
        Message::FloorSwitch(fs) => {
            tx.send(Message::FloorSwitch(fs))
                .await
                .expect("Failed to send message");
        }
        Message::ElevatorSwitch(es) => {
            tx.send(Message::ElevatorSwitch(es))
                .await
                .expect("Failed to send message");
        }
    }
}
