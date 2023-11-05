pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230314_213534_create_message_logs;
mod m20231023_172150_create_guild_config_table;
mod m20231023_204724_create_logging_table;
mod m20231105_163850_create_infraction_table;

mod models;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230314_213534_create_message_logs::Migration),
            Box::new(m20231023_172150_create_guild_config_table::Migration),
            Box::new(m20231023_204724_create_logging_table::Migration),
            Box::new(m20231105_163850_create_infraction_table::Migration),
        ]
    }
}
