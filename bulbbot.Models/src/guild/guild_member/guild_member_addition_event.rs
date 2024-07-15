use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, RoleId, User, UserId};

#[derive(Serialize, Deserialize)]
pub struct GuildMemberAdditionEventContent {
    #[serde(default)]
    pub user: User,
    #[serde(default)]
    pub roles: Vec<RoleId>,
    #[serde(default)]
    pub guild_id: GuildId,
}

#[derive(Serialize, Deserialize)]
pub struct GuildMemberAdditionEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: GuildMemberAdditionEventContent,
}
