//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "infractions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub guild_id: String,
    pub action: String,
    pub reason: String,
    pub target: String,
    pub target_id: String,
    pub moderator: String,
    pub moderator_id: String,
    pub timeout: Option<String>,
    pub active: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guilds::Entity",
        from = "Column::GuildId",
        to = "super::guilds::Column::GuildId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Guilds,
}

impl Related<super::guilds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guilds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}