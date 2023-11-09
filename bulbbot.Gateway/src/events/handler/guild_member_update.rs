use entity::prelude::{GuildConfigurations, Infractions};
use serenity::model::id::RoleId;
use serenity::model::Permissions;
use serenity::model::prelude::{GuildId, Member, MemberAction};
use serenity::prelude::Context;
use tracing::{error, warn};
use crate::events::event_handler::Handler;
use crate::events::models::log_type::LogType;
use crate::manger_container_structs::DatabaseMangerContainer;

impl Handler {
    pub async fn handle_guild_member_update(&self, ctx: Context, old_if_available: Option<Member>, mut new: Member) {
        let data = ctx.clone();
        let data_read = data.data.read().await;
        let guild_id = u64::from(new.guild_id);

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[EVENT/GUILD_MEMBER_UPDATE] Failed to get the 'database manager container'")
            .get()
            .expect("[EVENT/GUILD_MEMBER_UPDATE] The database connection is None");

        let db_guild_config = GuildConfigurations::find_by_guild_id(guild_id).one(db).await;
        if db_guild_config.is_err() {
            error!("[EVENT/GUILD_MEMBER_UPDATE] Database failed to get guild config: {:#?}", db_guild_config.err());
            return;
        }

        if old_if_available.is_none() {
            warn!("[EVENT/GUILD_MEMBER_UPDATE] Old member data is None. Cannot compare. GuildId: {:#?}", guild_id);
            return;
        }

        let old = old_if_available.unwrap();
        let auto_role = db_guild_config.unwrap().unwrap().auto_role;

        // Handle autorole adding with onboarding enabled
        if old.pending && !new.pending && auto_role.is_some() {
            let auto_role_id = auto_role.unwrap().parse::<u64>().unwrap();

            if let Err(_) = new.add_role(&ctx.http, RoleId(auto_role_id)).await
            {
                warn!("[EVENT/GUILD_MEMBER_ADD] Failed to add autorole: {:#?} to member: {:#?} in guild: {:#?}",
                    auto_role_id,
                    new.user.id.0,
                    new.guild_id.0,
                );
            }
        }

        // Handle manual unmute
        if old.communication_disabled_until.is_some() && new.communication_disabled_until.is_none() {
            let bot_perms = new
                .guild_id
                .member(&ctx.http, &ctx.cache.current_user_id())
                .await
                .unwrap()
                .permissions(&ctx.cache);

            if bot_perms.is_err() {
                error!("[EVENT/GUILD_MEMBER_UPDATE] Failed to fetch bot permissions in guild: {}", guild_id);
                return;
            }

            if !bot_perms.unwrap().contains(Permissions::VIEW_AUDIT_LOG) {
                warn!("[EVENT/GUILD_MEMBER_UPDATE] Bot does not have 'VIEW_AUDIT_LOG' permissions in guild: {}", guild_id);
                return;
            }

            let audit_logs = new
                .guild_id
                .audit_logs(&ctx.http, Some(MemberAction::Update as u8), None, None, Some(1u8))
                .await;

            if audit_logs.as_ref().is_err() || audit_logs.as_ref().unwrap().entries.is_empty() {
                error!("[EVENT/GUILD_MEMBER_UPDATE] Failed to fetch guild audit logs. Guild Id: {}", guild_id);
                return;
            }

            let audit_log_entry = audit_logs.as_ref().unwrap().entries.first().unwrap();

            if audit_log_entry.changes.is_none() || audit_log_entry.target_id.unwrap() != new.user.id.0 {
                error!("[EVENT/GUILD_MEMBER_UPDATE] Fetched the wrong audit log in guild id: {}", guild_id);
                return;
            }

            let log_message = String::from(
                format!(
                    "**{}** `({})` has been manually unmuted by **{}** `({})`",
                    new.user.name,
                    new.user.id,
                    audit_log_entry.user_id.to_user(&ctx.http).await.unwrap().name,
                    audit_log_entry.user_id,
                )
            );

            if let Err(why) = self.send_log(
                &ctx,
                &log_message,
                Some(GuildId(guild_id)),
                LogType::MuteRemove,
            ).await
            {
                error!("Failed to send log message in Guild: {:#?}, {:#?}", guild_id, why)
            }

            if let Err(why) = Infractions::insert_infraction(
                &db,
                guild_id,
                "Manual Unmute".to_string(),
                "No reason given".to_string(),
                new.user,
                new.guild_id.member(&ctx.http, &audit_log_entry.user_id).await.unwrap().user,
                None,
            ).await
            {
                error!("[EVENT/GUILD_MEMBER_ADD] Failed to add a new infraction record in guild: {} {}", guild_id, why);
                return;
            }
        }

        // Handle manual mute
        if old.communication_disabled_until.is_none() && new.communication_disabled_until.is_some() {
            // TODO: Implement manual mute
        }
    }
}