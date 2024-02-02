use darkredis::{Connection, ConnectionPool};
use std::env;

pub async fn init() -> Result<Connection, ()> {
    let redis_password = match env::var("REDIS_PASSWORD") {
        Ok(psw) => Some(psw),
        Err(_) => None,
    };

    let pool = ConnectionPool::create(
        env::var("REDIS_URL")
            .expect("[ENV] expected 'REDIS_URL' in the environment")
            .into(),
        redis_password.as_deref(),
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
