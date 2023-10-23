pub use crate::generated::guilds::*;
use crate::{generated::guild_loggings, prelude::GuildLoggings};
use sea_orm::{DatabaseConnection, EntityTrait, Set};

impl ActiveModel {}

impl GuildLoggings {
    pub async fn create_guild_loggings(db: &DatabaseConnection, guild_id: u64) {
        let guild_logging_model = guild_loggings::ActiveModel {
            guild_id: Set(guild_id.to_string()),
            ..Default::default()
        };

        let guild_logging = Self::insert(guild_logging_model);
        guild_logging.exec_without_returning(db).await.unwrap();
    }
}
