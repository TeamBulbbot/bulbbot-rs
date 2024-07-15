use serde::{Deserialize, Serialize};
use serenity::all::Message;

use crate::event_type::EventType;

#[derive(Serialize, Deserialize)]
pub struct MessageEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: Message,
}
