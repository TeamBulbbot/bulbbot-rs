use models::event_type::EventType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
}
