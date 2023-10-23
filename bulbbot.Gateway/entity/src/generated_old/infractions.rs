//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "infractions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub action: String,
    pub reason: String,
    pub target: String,
    #[sea_orm(column_name = "targetId")]
    pub target_id: String,
    pub moderator: String,
    #[sea_orm(column_name = "moderatorId")]
    pub moderator_id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "guildId")]
    pub guild_id: i32,
    pub timeout: Option<String>,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}