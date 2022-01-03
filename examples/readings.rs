use eyre::{bail, Report};
use octopower::{authenticate, consumption, MeterType};
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage:");
        eprintln!(
            "  {} <email address> <password> (electricity|gas) <MPxN> <meter serial>",
            args[0]
        );
        exit(1);
    }
    let email_address = &args[1];
    let password = &args[2];
    let meter_type = match args[3].as_ref() {
        "electricity" => MeterType::Electricity,
        "gas" => MeterType::Gas,
        t => bail!("Invalid meter type {}", t),
    };
    let mpxn = &args[4];
    let serial = &args[5];

    let token = authenticate(email_address, password).await?;

    println!("Auth token: {:?}", token);

    let consumption = consumption(&token, meter_type, mpxn, serial, 1, 100, None).await?;
    println!(
        "{:?} consumption: {}/{} records",
        meter_type,
        consumption.results.len(),
        consumption.count
    );
    for reading in &consumption.results {
        println!(
            "{}-{}: {}",
            reading.interval_start, reading.interval_end, reading.consumption
        );
    }
    println!("Previous: {:?}", consumption.previous);
    println!("Next: {:?}", consumption.next);

    Ok(())
}
