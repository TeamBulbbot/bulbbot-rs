use crate::events::event_handler::Handler;
use serenity::{model::event::ResumedEvent, prelude::Context};
use tracing::log::info;

impl Handler {
    pub async fn handle_resume(&self, _: Context, _event: ResumedEvent) {
        info!("Resumed connection to the Discord gateway");
    }
}
