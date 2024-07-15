use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId};

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEventContent {
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageDeleteEventContent,
}
