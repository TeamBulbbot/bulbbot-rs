use serenity::prelude::Context;

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn send_log(&self, _ctx: &Context) {}
}
