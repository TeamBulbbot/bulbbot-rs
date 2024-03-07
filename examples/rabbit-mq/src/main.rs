use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use serde::{Deserialize, Serialize};
use tokio::time;

#[derive(Serialize, Deserialize)]
enum EventType {
    Message,
    Delete,
}

#[derive(Serialize, Deserialize)]
struct Content {
    event_type: EventType,
    author_id: usize,
    author_username: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let queue_declare = QueueDeclareArguments::default()
        .queue(String::from("my-cool-queue"))
        .durable(false)
        .exclusive(false)
        .auto_delete(false)
        .finish();

    let (queue_name, message_count, consumer_count) =
        channel.queue_declare(queue_declare).await.unwrap().unwrap();

    println!(
        "message_count={}, consumer_count={}",
        message_count, consumer_count
    );

    let routing_key = "amqprs.example";
    let exchange_name = "amq.topic";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();

    let message = Content {
        author_id: 123,
        author_username: "mrphilip".to_string(),
        message: "Hello from Rust".to_string(),
        event_type: EventType::Message,
    };

    let content = serde_json::to_string(&message)?.into_bytes();

    let args = BasicPublishArguments::new(exchange_name, routing_key);

    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await
        .unwrap();

    time::sleep(time::Duration::from_secs(1)).await;
    channel.close().await.unwrap();
    connection.close().await.unwrap();

    Ok(())
}
