use serenity::model::gateway::Ready;
use serenity::prelude::Context;

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn handle_ready(&self, _: Context, ready: Ready) {
        println!(
            "[READY] Connected as {}#{} ({})",
            ready.user.name, ready.user.discriminator, ready.user.id
        );
        println!(
            "[READY] Session ID: {} with {} shard on version {}",
            ready.session_id,
            ready.shard.expect("failed to get shard count on 'READY'")[1],
            ready.version
        )
    }
}
