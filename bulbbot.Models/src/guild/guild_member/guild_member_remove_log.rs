use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, User, UserId};

#[derive(Serialize, Deserialize)]
pub struct RemoveGuildMemberLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub content: User,
}
