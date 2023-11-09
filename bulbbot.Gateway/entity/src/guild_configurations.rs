pub use crate::generated::guilds::*;
use crate::{generated::guild_configurations, prelude::GuildConfigurations};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select, Set};

impl ActiveModel {}

impl GuildConfigurations {
    pub async fn create_guild_configuration(db: &DatabaseConnection, guild_id: u64) {
        let guild_configuration_model = guild_configurations::ActiveModel {
            guild_id: Set(guild_id.to_string()),
            language_iso_code: Set(String::from("en")),
            ..Default::default()
        };

        let guild_configuration = Self::insert(guild_configuration_model);
        guild_configuration
            .exec_without_returning(db)
            .await
            .unwrap();
    }

    pub fn find_by_guild_id(guild_id: u64) -> Select<Self>{
        Self::find().filter(guild_configurations::Column::GuildId.eq(guild_id.to_string()))
    }
}

