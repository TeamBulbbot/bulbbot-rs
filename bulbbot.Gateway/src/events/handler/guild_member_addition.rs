use serenity::client::Context;
use serenity::model::prelude::Member;
use crate::events::event_handler::Handler;
use crate::manger_container_structs::DatabaseMangerContainer;
use entity::prelude::{Guilds};
use serenity::model::id::GuildId;
use tracing::error;
use crate::events::models::log_type::LogType;

impl Handler {
    pub async fn handle_guild_member_addition(&self, ctx: Context, new_member: Member) {
        let data = ctx.clone();
        let data_read = data.data.read().await;
        let guild_id = u64::from(new_member.guild_id);

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/GUILD_MEMBER_ADDITION] failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/GUILD_MEMBER_ADDITION] the database connection is None");

        let db_guild = Guilds::find_by_guild_id(guild_id).one(db).await;
        if db_guild.is_err() {
            error!("Database failed to get guild: {:#?}", db_guild.err());
            return;
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

        if let Err(why) = self.
            send_log(
                &ctx,
                &log_message,
                Some(GuildId(guild_id)),
                LogType::GuildMemberAddition,
            ).await
        {
            error!("Guild Id: {:#?} {:#?}", GuildId(guild_id), why)
        }
    }
}