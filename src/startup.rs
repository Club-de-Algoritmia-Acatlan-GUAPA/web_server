use actix_web::dev::Server;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use rocket::data;
use std::net::TcpListener;

use crate::queue_connection;
use crate::database_connection;

pub fn start_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    std::env::set_var("RUST_LOG", "debug");

    let server = HttpServer::new(move || {
        App::new()
            // .service(database::service())
            .configure(|c| queue_connection::config(c))
            .configure(|c| database_connection::config(c))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
