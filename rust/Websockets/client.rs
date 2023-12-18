use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

mod types;
use types::*;

async fn connect_client(id: usize) -> Result<()> {
    let url = Url::parse("ws://127.0.0.1:9001").unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    let message = serde_json::from_str::<LoadTestResponse>(&text).unwrap();
                    if let Some(caps) = Regex::new(r"Total time for [0-9]+ messages: ([0-9]+) ms")?
                        .captures(&message.message)
                    {
                        if let Some(matched) = caps.get(1) {
                            println!("Client {}: Total time: {} ms", id, matched.as_str());
                        }
                    } else {
                        ws_stream
                            .send(Message::Text(
                                serde_json::to_string(&LoadTestRequest {
                                    payload: "Acknowledged".into(),
                                })
                                .unwrap(),
                            ))
                            .await
                            .unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 9)]
async fn main() -> Result<()> {
    let total_clients: usize = std::env::var("TOTAL_CLIENTS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap();

    // Collect all client tasks in a vector
    let client_tasks: Vec<_> = (0..total_clients)
        .map(|id| {
            let client = connect_client(id);
            tokio::spawn(client)
        })
        .collect();

    // Await all client tasks concurrently
    for client_task in client_tasks {
        client_task.await??;
    }

    Ok(())
}
