mod events;
mod manger_container_structs;

use dotenv::dotenv;
use events::event_handler::Handler;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use manger_container_structs::ShardManagerContainer;
use serenity::prelude::*;
use std::env;
use tracing::log::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    info!(
        "{} on version: {} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );

    dotenv().ok();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_SCHEDULED_EVENTS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let token =
        env::var("DISCORD_TOKEN").expect("[ENV] expected 'DISCORD_TOKEN' in the environment");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("[STARTUP] error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

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
        "Connect to RabbitMQ - Status:{:#?} (via {:#?})",
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

    let payload = b"Hello world!";
    let confirm = rabbit_mq_channel
        .basic_publish(
            "",
            "bulbbot.gateway",
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        shard_manager.shutdown_all().await;

        rabbit_mq_channel
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ channel");
        rabbit_mq
            .close(200, "Normal shutdown")
            .await
            .expect("Failed to close Rabbit MQ connection");
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:#?}", why);
    }
}
