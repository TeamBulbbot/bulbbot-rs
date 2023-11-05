use crate::events::event_handler::Handler;
use crate::events::models::log_type::LogType;
use serenity::client::Context;
use serenity::model::id::GuildId;
use serenity::model::prelude::Member;
use tracing::error;

impl Handler {
    pub async fn handle_guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild_id = u64::from(new_member.guild_id);

        let mut log_message = String::new();
        log_message.push_str(
            format!(
                "**{}** `({})` has joined the server.\n**Account creation date:** <t:{:#?}:F> (<t:{:#?}:R>)",
                new_member.user.name,
                new_member.user.id,
                new_member.user.created_at().unix_timestamp(),
                new_member.user.created_at().unix_timestamp(),
            )
                .as_str()
        );

        if let Err(why) = self
            .send_log(
                &ctx,
                &log_message,
                Some(GuildId(guild_id)),
                LogType::GuildMemberAddition,
            )
            .await
        {
            error!("Guild Id: {:#?} {:#?}", GuildId(guild_id), why)
        }
    }
}
