use eyre::Report;
use octopower::{authenticate, electricity_consumption};

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let token = authenticate("username", "password").await?;

    println!("Auth token: {:?}", token);

    let consumption = electricity_consumption(&token, "MPAN", "serial").await?;
    println!(
        "Electricity consumption: {}/{} records",
        consumption.results.len(),
        consumption.count
    );
    for reading in &consumption.results {
        println!(
            "{}-{}: {}",
            reading.interval_start, reading.interval_end, reading.consumption
        );
    }
    println!("Next: {:?}", consumption.next);

    Ok(())
}
