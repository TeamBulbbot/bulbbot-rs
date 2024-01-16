use entity::prelude::Messages;
use entity::DatabaseConnection;
use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, GuildId, MessageId, UserId};

use super::bulbbot_user::BulbbotUser;

#[derive(Debug)]
pub struct BulbbotMessage {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub content: Option<String>,
    pub author: BulbbotUser,
}

#[derive(Debug)]
pub enum WhatChanged {
    Content,
}

impl BulbbotMessage {
    pub async fn convert_message(
        db: &DatabaseConnection,
        msg: Option<Message>,
        message_id: MessageId,
    ) -> Option<BulbbotMessage> {
        match msg {
            Some(msg) => Some(BulbbotMessage {
                id: msg.id,
                channel_id: msg.channel_id,
                content: Some(msg.clone().content),
                guild_id: msg.guild_id.unwrap(),
                author: BulbbotUser::create_user_from_message(&msg.clone()),
            }),
            None => Some({
                let fetched_message = Messages::fetch_message_one(&db, u64::from(message_id)).await;

                let msg = match fetched_message {
                    Ok(model) => model,
                    Err(_) => None,
                };

                if msg.is_none() {
                    return None;
                }
                let m = msg.unwrap();

                BulbbotMessage {
                    id: MessageId::new(m.message_id.parse().unwrap()),
                    channel_id: ChannelId::new(m.channel_id.parse().unwrap()),
                    content: m.content,
                    guild_id: GuildId::new(m.guild_id.parse().unwrap()),
                    author: BulbbotUser {
                        id: UserId::new(m.author_id.parse().unwrap()),
                        username: m.author_tag,
                    },
                }
            }),
        }
    }

    pub fn cmp(a: &BulbbotMessage, b: &BulbbotMessage) -> Vec<WhatChanged> {
        let mut what_changed: Vec<WhatChanged> = vec![];

        // prob is a better way of doing this, but cant bother
        if a.content != b.content {
            what_changed.push(WhatChanged::Content);
        }

        return what_changed;
    }
}
