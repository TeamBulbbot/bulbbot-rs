use crate::models::{Guilds, Infractions};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Infractions::Table)
                    .col(
                        ColumnDef::new(Infractions::GuildId)
                            .string()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("guild_id")
                            .from(Infractions::Table, Infractions::GuildId)
                            .to(Guilds::Table, Guilds::GuildId),
                    )
                    .col(ColumnDef::new(Infractions::InfId).not_null().auto_increment().integer().primary_key())
                    .col(ColumnDef::new(Infractions::Action).not_null().string())
                    .col(ColumnDef::new(Infractions::Reason).not_null().string())
                    .col(
                        ColumnDef::new(Infractions::Target)
                            .not_null()
                            .string()
                            .string_len(40),
                    )
                    .col(
                        ColumnDef::new(Infractions::TargetId)
                            .not_null()
                            .string()
                            .string_len(20),
                    )
                    .col(
                        ColumnDef::new(Infractions::Moderator)
                            .not_null()
                            .string()
                            .string_len(40),
                    )
                    .col(
                        ColumnDef::new(Infractions::ModeratorId)
                            .not_null()
                            .string()
                            .string_len(20),
                    )
                    .col(ColumnDef::new(Infractions::Timeout).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Infractions::Table).to_owned())
            .await
    }
}
