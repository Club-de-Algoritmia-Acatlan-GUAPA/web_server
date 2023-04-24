use std::net::TcpListener;
use web_server::configuration::get_configuration;
use web_server::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    startup::start_server(listener)?.await
}
