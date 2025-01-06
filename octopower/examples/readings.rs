// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use eyre::Report;
use octopower::{
    authenticate, get_account, get_consumption, get_standard_unit_rates, AuthToken, Grouping,
    MeterType,
};
use regex::Regex;
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

    let mut tariff_code: &str = "";
    let mut product_code: &str = "";
    let re = Regex::new(r#"[A-Z]+-[A-Z]+-\d{2}-\d{2}-\d{2}"#).unwrap();

    for property in &account.properties {
        println!("Property {}", property.address_line_1);
        for electricity_meter_point in &property.electricity_meter_points {
            println!("Electricity MPAN {}", electricity_meter_point.mpan);
            if let Some(last_agreement) = electricity_meter_point.agreements.last() {
                println!("Latest agreement {:?}", last_agreement);
                tariff_code = &last_agreement.tariff_code
            }
            // for agreements in &electricity_meter_point.agreements {
            //     println!("Agreement {:?}", agreements);
            // }
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

        println!("Tariff code = {}", tariff_code.to_string());
        // Match the regular expression against the input
        if let Some(captured) = re.find(tariff_code) {
            // Extract the matched substring
            product_code = &tariff_code[captured.start()..captured.end()];
        }
        println!("Extracted product code : {}", product_code);

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

        show_unit_rate(&token, MeterType::Electricity, product_code, tariff_code).await;
    }

    Ok(())
}

async fn show_consumption(token: &AuthToken, meter_type: MeterType, mpxn: &str, serial: &str) {
    match get_consumption(&token, meter_type, mpxn, serial, 0, 10, Some(Grouping::Day)).await {
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

async fn show_unit_rate(
    token: &AuthToken,
    meter_type: MeterType,
    product_code: &str,
    tariff_code: &str,
) {
    match get_standard_unit_rates(token, meter_type, product_code, tariff_code, 0, 10).await {
        Ok(unit_rates) => {
            println!(
                "{:?} unit rates: {}/{} records",
                meter_type,
                unit_rates.results.len(),
                unit_rates.count
            );
            for unit_rate in &unit_rates.results {
                println!(
                    "{}-{}: {}",
                    unit_rate.valid_from, unit_rate.valid_to, unit_rate.value_inc_vat
                );
            }
            println!("Previous: {:?}", unit_rates.previous);
            println!("Next: {:?}", unit_rates.next);
        }
        Err(e) => println!("Error getting unit rates page 1: {}", e),
    }
}
