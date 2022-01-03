mod config;

use config::{get_influxdb_client, Config};
use eyre::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let config = Config::from_file()?;
    let influxdb_client = get_influxdb_client(&config.influxdb)?;

    Ok(())
}
