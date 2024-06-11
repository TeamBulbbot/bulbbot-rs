use crate::events::{event_handler::Handler, models::log_type::LogType};
use serenity::{
    model::prelude::{ChannelId, GuildId, MessageId},
    prelude::Context,
};
use tracing::{debug, error};

impl Handler {
    pub async fn handle_message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
    }
}
