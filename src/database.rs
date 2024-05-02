use fred::prelude::*;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use crate::configuration::{DatabaseSettings, RedisSettings};

pub async fn get_postgres_pool(db_config: &DatabaseSettings) -> sqlx::PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_config.with_db());

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to run migrations");
    pool
}

pub fn get_redis_tls() -> fred::types::TlsConfig {
    let config: TlsConfig = TlsConnector::default_rustls().unwrap().into();
    config
}
pub async fn get_redis_pool(config: &RedisSettings) -> RedisPool {
    let redis_config =
        RedisConfig::from_url(config.uri.expose_secret().as_ref()).expect("Unable to parse config");
    //let mut builder = Builder::from_config(redis_config.clone());
    //builder.with_config(|config| {
    //    config.tls = Some(get_redis_tls());
    //    config.blocking = Blocking::Block;
    //    config.username = None;
    //    config.password = None;
    //    config.database = None;
    //});
    //let client = builder.build().unwrap();
    //client.wait_for_connect().await.unwrap();
    let pool = RedisPool::new(redis_config, None, None, None, 1).expect("Unable to set redi pool");

    pool.connect();
    pool.wait_for_connect()
        .await
        .expect("Unable to conenct to redis pool");

    pool
}
