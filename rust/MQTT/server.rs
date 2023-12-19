use anyhow::Result;
use rumqttc::{AsyncClient, MqttOptions, Publish, QoS};
use std::{env, process, time::Duration};
use tokio::{task::JoinHandle, time::sleep};

mod types;
use types::*;

async fn mqtt_publisher(
    total_clients: usize,
    total_messages: usize,
    payload_size: usize,
) -> Result<()> {
    let mut handles = Vec::new();

    for client_id in 0..total_clients {
        let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let mqttoptions =
                MqttOptions::new(format!("publisher_{}", client_id), "localhost", 1883);
            let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
            client
                .subscribe(format!("ack/client/{}", client_id), QoS::AtLeastOnce)
                .await?;

            let mut message_count = 0;
            let start_time = std::time::Instant::now();

            for _ in 0..total_messages {
                client
                    .publish(
                        &format!("client/{}", client_id),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&LoadTestResponse {
                            message: format!("{}", "x".repeat(payload_size)),
                        })
                        .unwrap(),
                    )
                    .await?;

                while let Ok(notification) = eventloop.poll().await {
                    if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) =
                        notification
                    {
                        let message = String::from_utf8(publish.payload.to_vec())?;
                        let ack = serde_json::from_str::<LoadTestRequest>(&message).unwrap();
                        println!("GOT A MESSAGE: {:?}", ack.payload); // Acknowledgment from the client
                        message_count += 1;
                        break;
                    }
                }
            }

            if message_count == total_messages {
                client
                    .publish(
                        &format!("client/{}", client_id),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&LoadTestResponse {
                            message: format!(
                                "Total time for {} messages: {:?} ms",
                                total_messages,
                                start_time.elapsed().as_millis()
                            ),
                        })
                        .unwrap(),
                    )
                    .await?;
                while let Ok(notification) = eventloop.poll().await {
                    if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) =
                        notification
                    {
                        break;
                    }
                }
            }

            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)] // Adjust the number of worker threads as needed
async fn main() -> Result<()> {
    let total_clients: usize = env::var("TOTAL_CLIENTS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Error parsing TOTAL_CLIENTS");
    let total_messages: usize = env::var("TOTAL_MESSAGES")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Error parsing TOTAL_MESSAGES");
    let payload_size: usize = env::var("PAYLOAD_SIZE")
        .unwrap_or_else(|_| "1500".to_string())
        .parse()
        .expect("Error parsing PAYLOAD_SIZE");

    mqtt_publisher(total_clients, total_messages, payload_size).await
}
