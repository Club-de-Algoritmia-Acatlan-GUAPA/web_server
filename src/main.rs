use anyhow::Result;
use tracing::info;
use web_server::{configuration::get_configuration, startup};
#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    info!("{}", &configuration.app.application_port);
    let server = startup::build(configuration).await?;
    server.await?;
    Ok(())
}
