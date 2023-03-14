use serenity::model::channel::Message;
use serenity::model::prelude::MessageUpdateEvent;
use serenity::prelude::Context;
use tracing::log::error;

use crate::events::event_handler::Handler;

use super::loggers::LogType;

#[derive(Debug)]
enum WhatChanged {
    Content,
    Pinned,
}

impl Handler {
    pub async fn handle_message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if old.is_none() || new.is_none() {
            return;
        }

        let mut what_changed: Vec<WhatChanged> = vec![];

        // shoulnt be possible to fail but you never know
        let old_msg = old.expect("[EVENT/MESSAGE_UPDATE] failed to unwarp 'old'");
        let new_msg = new.expect("[EVENT/MESSAGE_UPDATE] failed to unwarp 'new'");

        // prob is a better way of doing this, but cant bother
        if old_msg.content != new_msg.content {
            what_changed.push(WhatChanged::Content);
        }
        if old_msg.pinned != new_msg.pinned {
            what_changed.push(WhatChanged::Pinned);
        }

        // nothing that we are checking was changed
        if what_changed.len() == 0 {
            return;
        }

        let mut log_message = String::new();

        for change in &what_changed {
            let txt = match change {
                WhatChanged::Content => {
                    format!("**B:** {}\n**A:** {}", &old_msg.content, &new_msg.content)
                }
                WhatChanged::Pinned => {
                    if new_msg.pinned {
                        format!("🥬 **Message was pinned**\n```{}```", new_msg.content)
                    } else {
                        format!("🚗 **Message was unpinned**\n```{}```", new_msg.content)
                    }
                }
            };

            log_message.push_str(txt.as_str());
        }

        if let Err(why) = self
            .send_log(&ctx, &log_message, new_msg.guild_id, LogType::MessageUpdate)
            .await
        {
            error!("Error in {:#?} {:#?}", new_msg.guild_id, why);
        }
    }
}
