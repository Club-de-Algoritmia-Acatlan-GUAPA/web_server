use sqlx::postgres::PgPoolOptions;
use crate::configuration;

pub fn pool() -> sqlx::PgPool {
    let database_config = configuration::get_configuration()
        .expect("Failed getting config")
        .database;

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database_config.with_db())
}
