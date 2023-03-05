use std::env;

use entity::sea_orm::{ConnectOptions, Database, DbErr};
pub use entity::DatabaseConnection;
use tokio::sync::OnceCell;
use tracing::log::{self, info};

pub async fn init() -> Result<OnceCell<DatabaseConnection>, DbErr> {
    info!("Initializing the the database");
    let db_connection_url = env::var("DATABASE_URL")
        .expect("[STARTUP/DATABASE] unable to find `DATABASE_URL` in the env");

    let mut opt = ConnectOptions::new(db_connection_url.to_owned());
    opt
        //.connect_timeout(Duration::from_secs(8))
        //.acquire_timeout(Duration::from_secs(8))
        //.idle_timeout(Duration::from_secs(8))
        //.max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;

    /*
      TODO handle migrations here
    use migration::{Migrator, MigratorTrait};

      warn!("Running migrations on the database");
       /// Check the status of all migrations
       Migrator::status(&db).await.unwrap();
       */

    Ok(OnceCell::new_with(Some(db)))
}
