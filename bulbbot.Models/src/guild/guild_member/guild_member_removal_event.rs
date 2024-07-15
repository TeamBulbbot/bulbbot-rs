use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, User};

#[derive(Serialize, Deserialize)]
pub struct GuildMemberRemovalEventContent {
    pub guild_id: GuildId,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct GuildMemberRemovalEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: GuildMemberRemovalEventContent,
}
