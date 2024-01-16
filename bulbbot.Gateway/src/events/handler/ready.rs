use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::model::user::OnlineStatus::Online;
use serenity::prelude::Context;
use tracing::log::info;

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn handle_ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {} ({})", ready.user.name, ready.user.id);
        info!(
            "Session ID: {} with {} shard(s) on version {}",
            ready.session_id,
            ready
                .shard
                .expect("failed to get shard count on 'READY'")
                .total,
            ready.version
        );
        info!("Serving {} guild(s) on this shard", ready.guilds.len());

        let activity = ActivityData::watching("the light shine");

        ctx.set_presence(Some(activity), Online);
    }
}
