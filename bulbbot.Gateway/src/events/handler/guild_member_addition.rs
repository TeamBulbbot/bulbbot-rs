use crate::events::event_handler::Handler;

use serenity::client::Context;
use serenity::model::prelude::Member;

impl Handler {
    pub async fn handle_guild_member_addition(&self, ctx: Context, new_member: Member) {}
}
