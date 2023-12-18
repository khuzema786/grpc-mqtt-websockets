use anyhow::Result;
use futures::StreamExt;
use regex::Regex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod loadtest {
    tonic::include_proto!("loadtest");
}
use loadtest::load_tester_client::LoadTesterClient;
use loadtest::LoadTestRequest;

async fn connect_client(id: usize) -> Result<()> {
    let mut client = LoadTesterClient::connect("http://[::1]:50051").await?;
    let (tx, rx) = mpsc::channel(100000);
    let response = client.stream_payload(ReceiverStream::new(rx)).await?;
    let mut inbound = response.into_inner();

    while let Some(response) = inbound.next().await {
        let message = response?.message;
        if let Some(caps) =
            Regex::new(r"Total time for [0-9]+ messages: ([0-9]+) ms")?.captures(&message)
        {
            if let Some(matched) = caps.get(1) {
                println!("Client {}: Total time: {} ms", id, matched.as_str());
            }
        } else {
            tx.send(LoadTestRequest {
                payload: "Acknowledged".into(),
            })
            .await?;
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
