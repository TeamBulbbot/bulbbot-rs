use crate::events::event_handler::Handler;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::info;
use tracing::log::error;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {}
}
