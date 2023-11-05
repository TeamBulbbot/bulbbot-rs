use crate::events::event_handler::Handler;
use crate::events::models::log_type::LogType;
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::user::User;
use tracing::{error, warn};

impl Handler {
    pub async fn handle_guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        let member_joined_timestamp = match member_data_if_available {
            Some(member) => member
                .joined_at
                .expect(
                    "[EVENT/GUILD_MEMBER_REMOVAL] failed to get the 'database manager container'",
                )
                .unix_timestamp(),
            None => {
                warn!(
                    "[EVENT/GUILD_MEMBER_REMOVAL] Guild member data is None for user {}",
                    user.id
                );
                0
            }
        };

        let mut log_message = String::new();
        log_message.push_str(
            format!(
                "**{}** (`{}`) has left the server.\n**Joined server:** <t:{:#?}:F> (<t:{:#?}:R>)",
                user.name, user.id, member_joined_timestamp, member_joined_timestamp,
            )
            .as_str(),
        );

        if let Err(why) = self
            .send_log(
                &ctx,
                &log_message,
                Some(guild_id),
                LogType::GuildMemberRemoval,
            )
            .await
        {
            error!("Guild Id: {:#?} {:#?}", guild_id, why)
        }
    }
}
