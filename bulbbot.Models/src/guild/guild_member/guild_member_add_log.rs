use super::guild_member_addition_event::GuildMemberAdditionEventContent;
use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, UserId};

#[derive(Serialize, Deserialize)]
pub struct AddGuildMemberLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub content: GuildMemberAdditionEventContent,
}
