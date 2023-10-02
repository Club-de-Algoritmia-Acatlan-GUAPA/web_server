use sqlx::postgres::PgPoolOptions;

use crate::configuration::DatabaseSettings;

pub async fn get_pool(db_config: &DatabaseSettings) -> sqlx::PgPool {
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
