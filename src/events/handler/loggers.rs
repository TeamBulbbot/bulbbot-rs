use serenity::model::id::GuildId;
use serenity::prelude::Context;
use serenity::prelude::SerenityError;
use tracing::log::debug;

use crate::events::event_handler::Handler;

#[allow(dead_code)]
#[derive(Debug)]
pub enum LogType {
    MessageUpdate,
    MessageDelete,
}

impl Handler {
    pub async fn send_log(
        &self,
        ctx: &Context,
        log_message: &str,
        guild_id: Option<GuildId>,
        log_type: LogType,
    ) -> Result<(), SerenityError> {
        if guild_id.is_none() {
            return Ok(());
        }
        let channel_id: u64 = 990301499986968576;

        let channel = ctx.http.get_channel(channel_id).await?;
        let channel_guild = channel
            .guild()
            .expect("[LOGGER] failed to get guild on 'channel_guild'");

        let channel_perms =
            channel_guild.permissions_for_user(&ctx.cache, &ctx.cache.current_user_id())?;
        if !channel_perms.manage_webhooks() {
            debug!(
                "Missing permission 'manage_webhooks' in channel {} in guild {}",
                &channel_id, &channel_guild.guild_id
            );
        }

        let channel_webhooks = channel_guild.webhooks(&ctx.http).await?;

        let webhook = match channel_webhooks.first() {
            Some(hook) => hook.clone(),
            None => channel_guild
                .create_webhook_with_avatar(
                    &ctx.http,
                    "Bulbbot",
                    "https://github.com/TeamBulbbot/bulbbot/blob/master/assets/Logo.png?raw=true",
                )
                .await?,
        };

        webhook
            .execute(&ctx.http, true, |w| {
                w.content(log_message)
                .avatar_url("https://github.com/TeamBulbbot/bulbbot/blob/master/assets/Logo.png?raw=true")
                .username(format!("Bulbbot - {:#?}", log_type))
                    .allowed_mentions(|f| f.empty_parse().replied_user(false))
            })
            .await?;

        Ok(())
    }
}
