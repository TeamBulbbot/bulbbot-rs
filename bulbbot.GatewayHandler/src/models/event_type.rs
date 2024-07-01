use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Message,
    MessageUpdate,
    MessageDelete,
    GuildMemberAddition,
    GuildMemberRemoval,
}
