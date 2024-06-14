use lapin::Channel;
use serenity::{gateway::ShardManager, prelude::TypeMapKey};
use std::sync::Arc;

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub struct RabbitMQMangerContainer;
impl TypeMapKey for RabbitMQMangerContainer {
    type Value = Channel;
}
