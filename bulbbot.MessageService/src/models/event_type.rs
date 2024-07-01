use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EventType {
    Message,
    MessageUpdate,
    MessageDelete,
    GuildMemberAddition,
    GuildMemberRemoval,
}
