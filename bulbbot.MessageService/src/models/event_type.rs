use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventType {
    Message,
    MessageUpdate,
    MessageDelete,
    GuildMemberAddition,
    GuildMemberRemoval,
}
