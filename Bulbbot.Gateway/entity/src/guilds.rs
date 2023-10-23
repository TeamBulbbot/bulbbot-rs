pub use crate::generated::guilds::*;
use crate::generated::{guilds, prelude::Guilds};
use crate::prelude::{GuildConfigurations, GuildLoggings};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select, Set};

impl ActiveModel {}

impl Guilds {
    pub fn find_by_guild_id(guild_id: u64) -> Select<Self> {
        Self::find().filter(Column::GuildId.eq(guild_id.to_string()))
    }

    pub async fn create_guild(db: &DatabaseConnection, guild_id: u64) -> guilds::Model {
        let guild_model = guilds::ActiveModel {
            guild_id: Set(guild_id.to_string()),
        };

        let guild = Self::insert(guild_model);
        let db_guild = guild.exec_with_returning(db).await.unwrap();

        GuildConfigurations::create_guild_configuration(db, guild_id).await;
        GuildLoggings::create_guild_loggings(db, guild_id).await;

        return db_guild;
    }
}
