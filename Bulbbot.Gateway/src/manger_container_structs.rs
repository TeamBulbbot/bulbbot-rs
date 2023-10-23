use darkredis::Connection;
use entity::DatabaseConnection;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{Mutex, TypeMapKey};
use std::sync::Arc;
use tokio::sync::OnceCell;

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
