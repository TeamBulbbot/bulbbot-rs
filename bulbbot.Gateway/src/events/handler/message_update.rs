use crate::events::event_handler::Handler;
use crate::events::models::bulbbot_message::{BulbbotMessage, WhatChanged};
use crate::events::models::bulbbot_user::BulbbotUser;
use crate::events::models::log_type::LogType;
use crate::manger_container_structs::DatabaseMangerContainer;
use serenity::model::channel::Message;
use serenity::model::prelude::{GuildId, MessageUpdateEvent};
use serenity::prelude::Context;
use tracing::log::error;

impl Handler {
    pub async fn handle_message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        if event.guild_id.is_none() {
            return;
        }

        let data = ctx.clone();
        let data_read = data.data.read().await;

        let guild_id = u64::from(event.guild_id.unwrap());
        let author = match event.author {
            None => return,
            Some(author) => author,
        };

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/MESSAGE_UPDATE] failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/MESSAGE_UPDATE] the database connection is None");

        let old_message = BulbbotMessage::convert_message(&db, old.clone(), event.id).await;
        let new_message = Some(BulbbotMessage {
            id: event.id,
            content: event.content,
            guild_id: GuildId(guild_id),
            author: BulbbotUser::create_user_from_user(&author),
            channel_id: event.channel_id,
        });

        if old_message.is_none() || new_message.is_none() {
            return;
        }

        let old = old_message.expect("[EVENT/MESSAGE_UPDATE] failed to unwrap 'old_message'");
        let new = new_message.expect("[EVENT/MESSAGE_UPDATE] failed to unwrap 'new_message'");

        let changes = BulbbotMessage::cmp(&old, &new);

        // nothing that we are checking was changed
        if changes.len() == 0 {
            return;
        }

        let mut log_message = String::new();

        log_message.push_str(
            format!(
            "Message from **{}** `({})` has been updated in <#{}>\n**Message Id:** `{}`\n**Channel Id:** `{}`\n",
            author.name,
            author.id,
            event.channel_id,
            event.id,
            event.channel_id
        )
            .as_str(),
        );

        for change in &changes {
            let txt = match change {
                WhatChanged::Content => {
                    format!(
                        "**B:** {}\n**A:** {}",
                        &old.content
                            .clone()
                            .expect("[EVENT/MESSAGE_UPDATE] failed to unwrap 'old.content'"),
                        &new.content.clone().unwrap_or_else(|| String::new())
                    )
                }
            };

            log_message.push_str(txt.as_str());
        }

        if let Err(why) = self
            .send_log(
                &ctx,
                &log_message,
                Some(GuildId(guild_id)),
                LogType::MessageUpdate,
            )
            .await
        {
            error!("Guild id: {:#?} {:#?}", GuildId(guild_id), why);
        }
    }
}
