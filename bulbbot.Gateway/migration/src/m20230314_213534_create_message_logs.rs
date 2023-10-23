use crate::models::{Guilds, Messages};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::MessageId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Messages::ChannelId)
                            .string()
                            .not_null()
                            .string_len(20),
                    )
                    .col(
                        ColumnDef::new(Messages::AuthorId)
                            .string()
                            .not_null()
                            .string_len(20),
                    )
                    .col(
                        ColumnDef::new(Messages::AuthorTag)
                            .string()
                            .not_null()
                            .string_len(40),
                    )
                    .col(ColumnDef::new(Messages::Content).string().string_len(4000))
                    .col(ColumnDef::new(Messages::Embed).json())
                    .col(ColumnDef::new(Messages::Sticker).json())
                    .col(ColumnDef::new(Messages::Attachements).array(ColumnType::Text))
                    .col(ColumnDef::new(Messages::GuildId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("guild_id")
                            .from(Messages::Table, Messages::GuildId)
                            .to(Guilds::Table, Guilds::GuildId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}
