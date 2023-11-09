pub use crate::generated::infractions::*;
use crate::generated::prelude::Infractions;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult, Set};
use serenity::model::user::User;

impl ActiveModel {}

impl Infractions {
    pub async fn insert_infraction(
        db: &DatabaseConnection,
        guild_id: u64,
        action: String,
        reason: String,
        target: User,
        moderator: User,
        timeout: Option<String>,
    ) -> Result<InsertResult<ActiveModel>, DbErr> {
        let infraction = ActiveModel {
            guild_id: Set(guild_id.to_string()),
            action: Set(action),
            reason: Set(reason),
            target: Set(target.name),
            target_id: Set(target.id.to_string()),
            moderator: Set(moderator.name),
            moderator_id: Set(moderator.id.to_string()),
            timeout: Set(timeout),
            ..Default::default()
        };

        Self::insert(infraction).exec(db).await
    }
}
