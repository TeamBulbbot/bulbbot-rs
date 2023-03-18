use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select, Set};

pub use crate::generated::guilds::*;
use crate::generated::{guilds, prelude::Guilds};

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

        guild.exec_with_returning(db).await.unwrap()
    }
}
