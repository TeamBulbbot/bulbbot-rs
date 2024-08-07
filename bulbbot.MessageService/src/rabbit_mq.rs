use lapin::{
    options::QueueDeclareOptions, types::FieldTable, Channel, Connection, ConnectionProperties,
};
use tracing::info;

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
            "bulbbot.logging",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    (rabbit_mq, rabbit_mq_channel)
}
