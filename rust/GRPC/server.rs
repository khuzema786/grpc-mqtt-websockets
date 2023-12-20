use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod loadtest {
    tonic::include_proto!("loadtest");
}

use loadtest::load_tester_server::{LoadTester, LoadTesterServer};
use loadtest::{LoadTestRequest, LoadTestResponse};

#[derive(Default)]
pub struct MyLoadTester {}

#[tonic::async_trait]
impl LoadTester for MyLoadTester {
    type StreamPayloadStream =
        Pin<Box<dyn Stream<Item = Result<LoadTestResponse, Status>> + Send + Sync>>;

    async fn stream_payload(
        &self,
        request: Request<tonic::Streaming<LoadTestRequest>>,
    ) -> Result<Response<Self::StreamPayloadStream>, Status> {
        let (tx, rx) = mpsc::channel(100000);

        let total_messages: usize = std::env::var("TOTAL_MESSAGES")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap();
        let payload_size: usize = std::env::var("PAYLOAD_SIZE")
            .unwrap_or_else(|_| "1500".to_string())
            .parse()
            .unwrap();

        println!("Connection Successful");

        tokio::spawn(async move {
            let mut stream = request.into_inner();
            let mut message_count = 0;

            let start_time = std::time::Instant::now();

            for _ in 0..total_messages {
                tx.send(Ok(LoadTestResponse {
                    message: format!("{}", "x".repeat(payload_size)),
                }))
                .await
                .unwrap();
                while let Ok(Some(message)) = stream.message().await {
                    println!("GOT A MESSAGE: {:?}", message); // Acknowledgment from the client
                    message_count += 1;
                    break;
                }
            }

            if message_count == total_messages {
                tx.send(Ok(LoadTestResponse {
                    message: format!(
                        "Total time for {} messages: {:?} ms",
                        total_messages,
                        start_time.elapsed().as_millis()
                    ),
                }))
                .await
                .unwrap();
            }

            while let Ok(Some(message)) = stream.message().await {
                println!("GOT A MESSAGE: {:?}", message);
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)] // Adjust the number of worker threads as needed
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let load_tester = MyLoadTester::default();

    Server::builder()
        .add_service(LoadTesterServer::new(load_tester))
        .serve(addr)
        .await?;

    Ok(())
}
