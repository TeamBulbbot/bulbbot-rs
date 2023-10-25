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

    let mut redis = pool.get().await;

    let redis_ping = redis.ping().await;

    match redis_ping {
        Ok(_) => (),
        Err(err) => {
            panic!("[STARTUP/REDIS] Redis failed to answer to ping, is anything actually running on that URL? {:#?}", err);
        }
    }

    Ok(redis.to_owned())
}
