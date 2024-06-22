use serde::{Deserialize, Serialize};

use super::event_type::EventType;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    #[serde(default)]
    pub request_id: Uuid,
}
