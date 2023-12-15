mod constants;
mod database;
mod events;
mod manger_container_structs;
mod redis;

use dotenv::dotenv;
use events::event_handler::Handler;
use manger_container_structs::{
    DatabaseMangerContainer, RedisMangerContainer, ShardManagerContainer,
};
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

    let redis = redis::init()
        .await
        .expect("[STARTUP/REDIS] failed to setup redis");

    let database = database::init()
        .await
        .expect("[STARTUP/DATABASE] failed to setup the database");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_SCHEDULED_EVENTS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let token =
        env::var("DISCORD_TOKEN").expect("[ENV] expected 'DISCORD_TOKEN' in the environment");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .cache_settings(|cache| cache.max_messages(1000))
        .await
        .expect("[STARTUP] error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<RedisMangerContainer>(redis.clone());
        data.insert::<DatabaseMangerContainer>(database);
    }
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("[STARTUP] Could not register 'CTRL+C' handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:#?}", why);
    }
}
