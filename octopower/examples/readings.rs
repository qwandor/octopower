// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use eyre::Report;
use octopower::{authenticate, get_account, get_consumption, AuthToken, MeterType};
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage:");
        eprintln!("  {} <email address> <password> <account id>", args[0]);
        exit(1);
    }
    let email_address = &args[1];
    let password = &args[2];
    let account_id = &args[3];

    let token = authenticate(email_address, password).await?;

    let account = get_account(&token, account_id).await?;

    for property in &account.properties {
        println!("Property {}", property.address_line_1);
        for electricity_meter_point in &property.electricity_meter_points {
            println!("Electricity MPAN {}", electricity_meter_point.mpan);
            for meter in &electricity_meter_point.meters {
                println!("Meter serial {}", meter.serial_number);
                show_consumption(
                    &token,
                    MeterType::Electricity,
                    &electricity_meter_point.mpan,
                    &meter.serial_number,
                )
                .await;
            }
        }
        for gas_meter_point in &property.gas_meter_points {
            println!("Gas MPRN {}", gas_meter_point.mprn);
            for meter in &gas_meter_point.meters {
                println!("Meter serial {}", meter.serial_number);
                show_consumption(
                    &token,
                    MeterType::Gas,
                    &gas_meter_point.mprn,
                    &meter.serial_number,
                )
                .await;
            }
        }
    }

    Ok(())
}

async fn show_consumption(token: &AuthToken, meter_type: MeterType, mpxn: &str, serial: &str) {
    match get_consumption(&token, meter_type, mpxn, serial, 0, 10, None).await {
        Ok(consumption) => {
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
        }
        Err(e) => println!("Error getting consumption page 1 for meter: {}", e),
    }
}
