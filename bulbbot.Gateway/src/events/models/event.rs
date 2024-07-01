use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Message,
    MessageUpdate,
    MessageDelete,
    GuildMemberAddition,
    GuildMemberRemoval,
}
