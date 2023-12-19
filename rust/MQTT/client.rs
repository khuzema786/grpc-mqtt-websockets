use anyhow::Result;
use regex::Regex;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, Publish, QoS};
use std::env;

mod types;
use types::*;

async fn mqtt_client(client_id: usize) -> Result<()> {
    let mut mqttoptions = MqttOptions::new(format!("client_{}", client_id), "localhost", 1883);
    mqttoptions.set_keep_alive(60);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(format!("client/{}", client_id), QoS::AtLeastOnce)
        .await?;

    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
            let message = serde_json::from_str::<LoadTestResponse>(&String::from_utf8(
                publish.payload.to_vec(),
            )?)
            .unwrap();
            if let Some(caps) = Regex::new(r"Total time for [0-9]+ messages: ([0-9]+) ms")?
                .captures(&message.message)
            {
                if let Some(matched) = caps.get(1) {
                    println!("Client {}: Total time: {} ms", client_id, matched.as_str());
                }
            }
            let ack_topic = format!("ack/client/{}", client_id);
            client
                .publish(
                    &ack_topic,
                    QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(&LoadTestRequest {
                        payload: "Acknowledged".into(),
                    })
                    .unwrap(),
                )
                .await?;
        }
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<()> {
    let total_clients: usize = std::env::var("TOTAL_CLIENTS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()?;

    // Collect all client tasks in a vector
    let client_tasks: Vec<_> = (0..total_clients)
        .map(|id| {
            let client = mqtt_client(id);
            tokio::spawn(client)
        })
        .collect();

    // Await all client tasks concurrently
    for client_task in client_tasks {
        client_task.await??;
    }

    Ok(())
}
