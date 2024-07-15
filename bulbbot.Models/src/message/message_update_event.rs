use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::model;

#[derive(Serialize, Deserialize)]
pub struct MessageUpdateEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: model::prelude::MessageUpdateEvent,
}
