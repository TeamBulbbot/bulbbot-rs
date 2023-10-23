use crate::models::{GuildConfigurations, Guilds};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GuildConfigurations::Table)
                    .col(
                        ColumnDef::new(GuildConfigurations::GuildId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GuildConfigurations::LanguageISOCode)
                            .string()
                            .not_null()
                            .string_len(4),
                    )
                    .col(
                        ColumnDef::new(GuildConfigurations::HasPremium)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(GuildConfigurations::AutoRole)
                            .string()
                            .string_len(20),
                    )
                    .col(
                        ColumnDef::new(GuildConfigurations::ActionsOnInfo)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(GuildConfigurations::RolesOnLeave)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(GuildConfigurations::QuickReasons).array(ColumnType::Text))
                    .col(
                        ColumnDef::new(GuildConfigurations::ManualNickNameInfs)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("guild_id")
                            .from(GuildConfigurations::Table, GuildConfigurations::GuildId)
                            .to(Guilds::Table, Guilds::GuildId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GuildConfigurations::Table).to_owned())
            .await
    }
}
