use futures_util::{SinkExt, StreamExt};
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

mod types;
use types::*;

async fn handle_connection(stream: tokio::net::TcpStream) {
    let mut websocket = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let total_messages: usize = std::env::var("TOTAL_MESSAGES")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap();
    let payload_size: usize = std::env::var("PAYLOAD_SIZE")
        .unwrap_or_else(|_| "1500".to_string())
        .parse()
        .unwrap();

    // println!("Connection Successful");

    let start_time = Instant::now();

    for _ in 0..total_messages {
        websocket
            .send(Message::Text(
                serde_json::to_string(&LoadTestResponse {
                    message: format!("{}", "x".repeat(payload_size)),
                })
                .unwrap(),
            ))
            .await
            .unwrap();

        if let Some(Ok(Message::Text(ack))) = websocket.next().await {
            let ack = serde_json::from_str::<LoadTestRequest>(&ack).unwrap();
            // println!("GOT A MESSAGE: {:?}", ack.payload); // Acknowledgment from the client
        }
    }

    websocket
        .send(Message::Text(
            serde_json::to_string(&LoadTestResponse {
                message: format!(
                    "Total time for {} messages: {:?} ms",
                    total_messages,
                    start_time.elapsed().as_millis()
                ),
            })
            .unwrap(),
        ))
        .await
        .unwrap();

    if let Some(Ok(Message::Text(ack))) = websocket.next().await {
        let ack = serde_json::from_str::<LoadTestRequest>(&ack).unwrap();
        println!("GOT A MESSAGE: {:?}", ack.payload); // Acknowledgment from the client
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("WebSocket server listening on ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        spawn(handle_connection(stream));
    }
}
