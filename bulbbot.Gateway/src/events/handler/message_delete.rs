use crate::{
    events::{
        event_handler::Handler,
        models::{bulbbot_message::BulbbotMessage, log_type::LogType},
    },
    manger_container_structs::DatabaseMangerContainer,
};
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
        let data = ctx.clone();
        let data_read = data.data.read().await;

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/MESSAGE_DELETE] failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/MESSAGE_DELETE] the database connection is None");

        let msg = match BulbbotMessage::convert_message(&db, None, deleted_message_id).await {
            Some(msg) => msg,
            None => {
                debug!(
                    "Message '{}' from {} in {:#?} does not exist in database",
                    &deleted_message_id, &channel_id, &guild_id
                );
                return;
            }
        };

        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let log_message = String::from(format!(
            "Message from **{}** `({})` has been deleted in <#{}>\n**Message Id:** `{}`\n**Channel Id:** `{}`\n**Content:** {}",
            msg.author.username,
            msg.author.id,
            msg.channel_id,
            msg.id,
            msg.channel_id,
            msg.content.expect("[EVENT/MESSAGE_DELETE] failed to unwrap content of message")
        ));

        if let Err(why) = self
            .send_log(
                &ctx,
                &log_message,
                Some(msg.guild_id),
                LogType::MessageDelete,
            )
            .await
        {
            error!("Guild id: {:#?} {:#?}", msg.guild_id, why);
        }
    }
}
