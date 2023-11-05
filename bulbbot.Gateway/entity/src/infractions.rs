pub use crate::generated::infractions::*;
use crate::generated::{infractions, prelude::Infractions};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult, Set};

impl ActiveModel {}

impl Infractions {
    pub async fn insert_infraction(
        db: &DatabaseConnection,
        guild_id: u64,
    ) -> Result<InsertResult<infractions::ActiveModel>, DbErr> {
        todo!("Fill this out if you want to use it");

        let infraction = infractions::ActiveModel {
            guild_id: Set(guild_id.to_string()),
            ..Default::default()
        };

        Self::insert(infraction).exec(db).await
    }
}
