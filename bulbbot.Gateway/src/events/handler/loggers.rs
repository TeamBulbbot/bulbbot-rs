use crate::constants::ONE_HOUR;
use crate::events::event_handler::Handler;
use crate::events::models::log_type::database_column;
use crate::events::models::log_type::database_column_name;
use crate::events::models::log_type::LogType;
use crate::manger_container_structs::DatabaseMangerContainer;
use crate::manger_container_structs::RedisMangerContainer;
use serenity::model::id::GuildId;
use serenity::prelude::Context;
use serenity::prelude::SerenityError;
use std::borrow::BorrowMut;
use std::str;
use tracing::error;
use tracing::log::debug;

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
        let guild_id = u64::from(guild_id.unwrap());

        let data = ctx.clone();
        let data_read = data.data.read().await;

        let mut redis_connection = data_read
            .get::<RedisMangerContainer>()
            .expect("[LOGGER] failed to get the 'RedisMangerContainer'")
            .clone();
        let redis = redis_connection.borrow_mut();
        let redis_key = format!("{}:{}", database_column_name(&log_type), guild_id);

        let db = data_read
            .get::<DatabaseMangerContainer>()
            .expect("[LOGGER] failed to get the 'database manager container'")
            .get()
            .expect("[LOGGER] the database connection is None");

        let channel_id: Option<u64> = match redis.get(&redis_key).await {
            Ok(v) => match v {
                Some(value) => {
                    let content = str::from_utf8(&value).expect("[LOGGER] Invalid UTF-8 sequence");
                    Some(
                        content
                            .parse()
                            .expect("[LOGGER] Failed to convert content to u64"),
                    )
                }
                None => {
                    let logging = database_column(db, guild_id, &log_type).await;

                    let correct_format = match logging {
                        Some(c_id) => Some(
                            c_id.parse::<u64>()
                                .expect("[LOGGER] Failed to convert 'channel_id' to 'u64'"),
                        ),
                        None => None,
                    };

                    if correct_format.is_some() {
                        let _ = redis
                            .set_and_expire_ms(
                                &redis_key,
                                correct_format.unwrap().to_string(),
                                ONE_HOUR,
                            )
                            .await;
                    }

                    correct_format
                }
            },
            Err(_) => {
                error!("[LOGGER] Failed to get '{}' from Redis", &redis_key);
                None
            }
        };

        if channel_id.is_none() {
            return Ok(());
        }

        let channel = ctx.http.get_channel(channel_id.unwrap()).await?;
        let channel_guild = channel
            .guild()
            .expect("[LOGGER] failed to get guild on 'channel_guild'");

        let channel_perms =
            channel_guild.permissions_for_user(&ctx.cache, &ctx.cache.current_user_id())?;
        if !channel_perms.manage_webhooks() {
            debug!(
                "[LOGGER] Missing permission 'manage_webhooks' in channel {} in guild {}",
                &channel_id.unwrap(),
                &channel_guild.guild_id
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
