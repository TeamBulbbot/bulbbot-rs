use serenity::model::channel::Message;
use serenity::model::prelude::MessageUpdateEvent;
use serenity::prelude::Context;

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn handle_message_update(
        &self,
        _ctx: Context,
        _old: Option<Message>,
        _new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        //println!("Old: {:#?}", old.is_none());
        //println!("New: {:#?}", new.is_none());

        //println!("event: {:#?}", event)
    }
}
