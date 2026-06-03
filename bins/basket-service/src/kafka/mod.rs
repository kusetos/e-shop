use std::sync::Arc;

use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use serde::Deserialize;

use crate::repository::BasketRepository;

const TOPIC: &str = "order-created";

#[derive(Deserialize)]
struct OrderCreatedEvent {
    order_id: i32,
    user_id:  i32,
}

pub async fn start_consumer(brokers: String, repo: Arc<BasketRepository>) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", "basket-service")
        .set("auto.offset.reset", "earliest")
        .set("broker.address.family", "v4")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer
        .subscribe(&[TOPIC])
        .expect("Failed to subscribe to topic");

    tracing::info!("kafka consumer: listening on topic '{TOPIC}'");

    loop {
        match consumer.recv().await {
            Err(e) => tracing::error!("kafka recv error: {e}"),
            Ok(msg) => {
                let Some(payload) = msg.payload() else { continue };

                match serde_json::from_slice::<OrderCreatedEvent>(payload) {
                    Err(e) => tracing::error!("kafka deserialize error: {e}"),
                    Ok(event) => {
                        tracing::info!(
                            "kafka: OrderCreated order_id={} user_id={}",
                            event.order_id, event.user_id
                        );
                        if let Err(e) = repo.clear(event.user_id).await {
                            tracing::error!(
                                "failed to clear basket user_id={}: {e}",
                                event.user_id
                            );
                        }
                    }
                }
            }
        }
    }
}
