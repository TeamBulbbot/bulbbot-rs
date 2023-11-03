use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::user::User;
use tracing::{error, warn};
use crate::events::event_handler::Handler;
use crate::events::models::log_type::LogType;
use crate::manger_container_structs::DatabaseMangerContainer;
use entity::prelude::{Guilds};

impl Handler {
    pub async fn handle_guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, member_data_if_available: Option<Member>) {
        let data = ctx.clone();
        let data_read = data.data.read().await;

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/GUILD_MEMBER_REMOVAL] failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/GUILD_MEMBER_REMOVAL] database connection is None");

        let db_guild = Guilds::find_by_guild_id(u64::from(guild_id)).one(db).await;
        if db_guild.is_err() {
            error!("Database failed to get guild: {:#?}", db_guild.err());
            return;
        }

        let member_joined_timestamp = match member_data_if_available {
            Some(member) => member
                .joined_at
                .expect("[EVENT/GUILD_MEMBER_REMOVAL] failed to get the 'database manager container'")
                .unix_timestamp(),
            None => {
                warn!("[EVENT/GUILD_MEMBER_REMOVAL] Guild member data is None for user {}", user.id);
                0
            }
        };

        let mut log_message = String::new();
        log_message.push_str(
            format!(
                "**{}** (`{}`) has left the server.\n**Joined server:** <t:{:#?}:F> (<t:{:#?}:R>)",
                user.name,
                user.id,
                member_joined_timestamp,
                member_joined_timestamp,
            )
                .as_str()
        );

        if let Err(why) = self
            .send_log(
                &ctx,
                &log_message,
                Some(guild_id),
                LogType::GuildMemberRemoval
            ).await
        {
            error!("Guild Id: {:#?} {:#?}", guild_id, why)
        }
    }
}