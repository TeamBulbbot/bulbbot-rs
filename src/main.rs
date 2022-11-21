mod db;
mod events;

use dotenv::dotenv;
use events::event_handler::Handler;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    dotenv().ok().expect("[STARTUP] faild to load .env");

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
        .await
        .expect("[STARTUP] error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("[STARTUP] Could not register 'CTRL+C' handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        eprintln!("[STARTUP] client error: {:#?}", why);
    }
}
