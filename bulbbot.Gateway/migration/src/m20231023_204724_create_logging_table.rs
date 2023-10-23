use crate::models::{GuildLoggings, Guilds};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GuildLoggings::Table)
                    .col(
                        ColumnDef::new(GuildLoggings::GuildId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("guild_id")
                            .from(GuildLoggings::Table, GuildLoggings::GuildId)
                            .to(Guilds::Table, Guilds::GuildId),
                    )
                    .col(ColumnDef::new(GuildLoggings::ModAction).string())
                    .col(ColumnDef::new(GuildLoggings::AutoMod).string())
                    .col(ColumnDef::new(GuildLoggings::Message).string())
                    .col(ColumnDef::new(GuildLoggings::Role).string())
                    .col(ColumnDef::new(GuildLoggings::Member).string())
                    .col(ColumnDef::new(GuildLoggings::Channel).string())
                    .col(ColumnDef::new(GuildLoggings::Thread).string())
                    .col(ColumnDef::new(GuildLoggings::JoinLeave).string())
                    .col(ColumnDef::new(GuildLoggings::Invite).string())
                    .col(ColumnDef::new(GuildLoggings::Banpool).string())
                    .col(ColumnDef::new(GuildLoggings::Other).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GuildLoggings::Table).to_owned())
            .await
    }
}
