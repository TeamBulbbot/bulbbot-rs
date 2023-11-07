use entity::prelude::GuildConfigurations;
use crate::events::event_handler::Handler;
use crate::events::models::log_type::LogType;
use serenity::client::Context;
use serenity::model::id::{GuildId, RoleId};
use serenity::model::prelude::Member;
use tracing::{error, warn};
use crate::manger_container_structs::DatabaseMangerContainer;

impl Handler {
    pub async fn handle_guild_member_addition(&self, ctx: Context, mut new_member: Member) {
        let data = ctx.clone();
        let data_read = data.data.read().await;
        let guild_id = u64::from(new_member.guild_id);

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/GUILD_MEMBER_ADDITION] Failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/GUILD_MEMBER_ADDITION] The database connection is None");

        let guild_config = GuildConfigurations::find_by_guild_id(guild_id).one(db).await;
        if guild_config.is_err() {
            error!("[EVENT/GUILD_MEMBER_ADDITION] Database failed to get guild: {:#?}", guild_config.err());
            return;
        }

        let auto_role = guild_config.unwrap().unwrap().auto_role;

        if new_member.communication_disabled_until.is_none() && !auto_role.is_none() {
            let auto_role_id = auto_role.unwrap().parse::<u64>().unwrap();

            if let Err(_) = new_member.add_role(&ctx.http, RoleId(auto_role_id)).await
            {
                warn!("[EVENT/GUILD_MEMBER_ADD] Failed to add autorole: {:#?} to member: {:#?} in guild: {:#?}",
                    auto_role_id,
                    new_member.user.id.0,
                    new_member.guild_id.0,
                );
            }
        }

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
