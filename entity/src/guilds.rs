use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select};

pub use crate::generated::guilds::*;
use crate::generated::prelude::Guilds;

impl ActiveModel {}

impl Guilds {
    pub fn find_by_guild_id(guild_id: u64) -> Select<Self> {
        Self::find().filter(Column::GuildId.eq(guild_id.to_string()))
    }
}
