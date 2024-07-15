use crate::event_type::EventType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageUpdateLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: Option<i64>,
    pub channel_id: i64,
    pub message_id: i64,
    pub before_content: Option<String>,
    pub after_content: Option<String>,
}
