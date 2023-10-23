use darkredis::{Connection, ConnectionPool};
use std::env;

pub async fn init() -> Result<Connection, ()> {
    //  sudo service redis-server start
    let pool = ConnectionPool::create(
        env::var("REDIS_URL")
            .expect("[ENV] expected 'REDIS_URL' in the environment")
            .into(),
        None,
        16,
    )
    .await
    .expect("[STARTUP/REDIS] failed to validate the redis connection url");

    let redis = pool.get().await;

    Ok(redis.to_owned())
}
