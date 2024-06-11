use serenity::{gateway::ShardManager, prelude::TypeMapKey};
use std::sync::Arc;

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}
