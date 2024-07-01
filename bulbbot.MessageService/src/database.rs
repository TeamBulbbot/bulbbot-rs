use diesel::pg::PgConnection;
use diesel::r2d2;
use std::env;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failde to create establish database connection")
}
