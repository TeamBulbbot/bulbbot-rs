use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn handle_message(&self, _: Context, msg: Message) {
        println!("{:#?}", msg);
    }
}
