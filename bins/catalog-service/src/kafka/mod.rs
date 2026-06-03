use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use serde::Deserialize;

use crate::repository::product::ProductRepository;

const TOPIC: &str = "order-created";

#[derive(Deserialize)]
struct OrderCreatedEvent {
    items: Vec<OrderCreatedItem>,
}

#[derive(Deserialize)]
struct OrderCreatedItem {
    product_id: i32,
    quantity:   i32,
}

pub async fn start_consumer(brokers: String, repo: ProductRepository) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", "catalog-service")
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
                        for item in &event.items {
                            if let Err(e) = repo.decrement_stock(item.product_id, item.quantity).await {
                                tracing::error!(
                                    "failed to decrement stock product_id={}: {e}",
                                    item.product_id
                                );
                            } else {
                                tracing::info!(
                                    "stock decremented product_id={} by {}",
                                    item.product_id, item.quantity
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
