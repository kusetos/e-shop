use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;

const TOPIC: &str = "order-created";

#[derive(Serialize)]
pub struct OrderCreatedEvent {
    pub order_id: i32,
    pub user_id: i32,
    pub items: Vec<OrderCreatedItem>,
}

#[derive(Serialize)]
pub struct OrderCreatedItem {
    pub product_id: i32,
    pub quantity: i32,
}

pub struct EventProducer {
    inner: FutureProducer,
}

impl EventProducer {
    pub fn new(brokers: &str) -> Self {
        let inner = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("broker.address.family", "v4")
            .create()
            .expect("Failed to create Kafka producer");

        Self { inner }
    }

    pub async fn order_created(&self, event: &OrderCreatedEvent) {
        let payload = match serde_json::to_string(event) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("kafka serialize error: {e}");
                return;
            }
        };

        let key = event.order_id.to_string();
        let record = FutureRecord::to(TOPIC).payload(&payload).key(&key);

        match self
            .inner
            .send(record, std::time::Duration::from_secs(5))
            .await
        {
            Ok(_) => tracing::info!("kafka: OrderCreated order_id={}", event.order_id),
            Err((e, _)) => tracing::error!("kafka send error: {e}"),
        }
    }
}
