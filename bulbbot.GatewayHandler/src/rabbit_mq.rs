use lapin::types::{AMQPValue, FieldTable};
use lapin::{options::QueueDeclareOptions, Channel, Connection, ConnectionProperties};
use opentelemetry::propagation::Extractor;
use tracing::info;

pub struct RabbitMqExtractor<'a>(pub &'a FieldTable);

impl<'a> Extractor for RabbitMqExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.inner().get(key).and_then(|header_value| {
            if let AMQPValue::LongString(header_value) = header_value {
                std::str::from_utf8(header_value.as_bytes()).ok()
            } else {
                None
            }
        })
    }

    fn keys(&self) -> Vec<&str> {
        self.0.inner().keys().map(|k| k.as_str()).collect()
    }
}
pub async fn connect() -> (Connection, Channel) {
    let rabbit_mq_addr = format!(
        "amqp://{}:{}@{}",
        std::env::var("RABBIT_MQ_USERNAME").unwrap_or(String::from("guest")),
        std::env::var("RABBIT_MQ_PASSWORD").unwrap_or(String::from("guest")),
        std::env::var("RABBIT_MQ_URL").unwrap_or(String::from("localhost:5672"))
    );

    let rabbit_mq = Connection::connect(&rabbit_mq_addr, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ server");

    info!(
        "Connected to RabbitMQ - Status:{:#?} (via {:#?})",
        &rabbit_mq.status().state(),
        &rabbit_mq.status().username()
    );

    let rabbit_mq_channel = rabbit_mq
        .create_channel()
        .await
        .expect("Failed to create RabbitMQ channel");

    let _rabbit_mq_queue = rabbit_mq_channel
        .queue_declare(
            "bulbbot.gateway",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    (rabbit_mq, rabbit_mq_channel)
}
