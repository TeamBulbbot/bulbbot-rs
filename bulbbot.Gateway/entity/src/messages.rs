pub use crate::generated::messages::*;
use crate::generated::{messages, prelude::Messages};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter, Set,
};
use serenity::model::prelude::Message;

impl ActiveModel {}

impl Messages {
    pub async fn insert_message(
        db: &DatabaseConnection,
        msg: &Message,
        guild_id: u64,
    ) -> Result<InsertResult<messages::ActiveModel>, DbErr> {
        let content = Set(Some(msg.content.clone()).filter(|s| !s.is_empty()));

        let message = messages::ActiveModel {
            message_id: Set(msg.id.to_string()),
            channel_id: Set(msg.channel_id.to_string()),
            author_id: Set(msg.author.id.to_string()),
            author_tag: Set(msg.author.tag()),
            content,
            // TODO, add embed, attachements and stickers
            embed: Set(None),
            attachements: Set(None),
            sticker: Set(None),
            guild_id: Set(guild_id.to_string()),
        };

        Self::insert(message).exec(db).await
    }

    pub async fn fetch_message_one(
        db: &DatabaseConnection,
        message_id: u64,
    ) -> Result<Option<messages::Model>, DbErr> {
        Self::find()
            .filter(Column::MessageId.eq(message_id.to_string()))
            .one(db)
            .await
    }
}
