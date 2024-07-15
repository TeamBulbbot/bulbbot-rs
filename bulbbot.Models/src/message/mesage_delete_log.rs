use crate::event_type::EventType;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId};

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteLog {
    pub event: EventType,
    pub shard_id: u32,
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub content: Option<String>,
}
