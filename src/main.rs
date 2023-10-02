use anyhow::Result;
use web_server::{configuration::get_configuration, startup};
#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("{}", &configuration.app.application_port);
    let server = startup::build(configuration).await?;
    server.await?;
    Ok(())
}
