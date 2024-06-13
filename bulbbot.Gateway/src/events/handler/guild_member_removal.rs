use crate::events::event_handler::Handler;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::user::User;

impl Handler {
    pub async fn handle_guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
    }
}
