mod database;
mod events;

use darkredis::{Connection, ConnectionPool};
use dotenv::dotenv;
use entity::DatabaseConnection;
use events::event_handler::Handler;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tracing::log::{error, info};

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct RedisMangerContainer;
impl TypeMapKey for RedisMangerContainer {
    type Value = Connection;
}

pub struct DatabaseMangerContainer;
impl TypeMapKey for DatabaseMangerContainer {
    type Value = OnceCell<DatabaseConnection>;
}

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

    dotenv().ok().expect("[STARTUP] failed to load .env");

    //  sudo service redis-server start
    let pool = ConnectionPool::create(
        env::var("REDIS_URL")
            .expect("[STARTUP] expected 'REDIS_URL' in the environment")
            .into(),
        None,
        16,
    )
    .await
    .expect("[STARTUP] failed to validate the redis connection url");
    let redis_conn = pool.get().await;

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
        env::var("DISCORD_TOKEN").expect("[STARTUP] expected 'DISCORD_TOKEN' in the environment");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .cache_settings(|cache| cache.max_messages(1000))
        .await
        .expect("[STARTUP] error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<RedisMangerContainer>(redis_conn.clone());
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
