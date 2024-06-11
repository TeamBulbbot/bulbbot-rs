use crate::events::event_handler::Handler;

use serenity::model::channel::Message;
use serenity::model::prelude::MessageUpdateEvent;
use serenity::prelude::Context;

impl Handler {
    pub async fn handle_message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
    }
}
