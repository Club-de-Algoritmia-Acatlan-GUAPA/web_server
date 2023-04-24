use sqlx::{postgres::PgPoolOptions};
use actix_web::{web };
use crate::configuration;

// TODO : mover rocket.toml a .env file con figment (para evitar tener
// expuestos las contrasenias de postgres
// https://api.rocket.rs/v0.5-rc/rocket_sync_db_pools/index.html#provided

pub fn config(config: &mut web::ServiceConfig) {
    let database_config = configuration::get_configuration()
    .expect("Failed getting config")
    .database;


    let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(database_config.with_db());

    let data_pool = web::Data::new(pool);

    config.app_data(data_pool.clone());
}