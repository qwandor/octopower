// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use enphase_local::{
    Envoy,
    production::{Device, DeviceType, MeasurementType},
};
use eyre::Report;
use reqwest::Url;
use std::{process::exit, thread::sleep, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("  {} <base URL> <auth token>", args[0]);
        exit(1);
    }
    let base_url = &args[1];
    let auth_token = &args[2];

    let envoy = Envoy::new(Url::parse(base_url)?, auth_token);
    println!("Home: {:#?}", envoy.home().await?);
    println!("Inventory: {:#?}", envoy.inventory(true).await?);
    println!("Production: {:#?}", envoy.production().await?);
    println!("IVP meter readings: {:#?}", envoy.meter_readings().await?);
    println!("IVP meter reports: {:#?}", envoy.meter_reports().await?);
    println!("Inverters: {:#?}", envoy.inverters().await?);

    let mut last_inverters = Vec::new();
    loop {
        let production = envoy.production().await?;
        for device in &production.production {
            print_stats(device);
        }
        for device in &production.consumption {
            print_stats(device);
        }

        let inverters = envoy.inverters().await?;
        if inverters != last_inverters {
            for inverter in &inverters {
                println!(
                    "{} Inverter {} producing {} W (max {} W)",
                    inverter.last_report_date,
                    inverter.serial_number,
                    inverter.last_report_watts,
                    inverter.max_report_watts,
                );
            }
            let inverter_production_sum = inverters
                .iter()
                .map(|inverter| inverter.last_report_watts)
                .sum::<u32>();
            println!("Total: {inverter_production_sum} W");
        }
        last_inverters = inverters;

        sleep(Duration::from_secs(2));
    }
}

fn print_stats(device: &Device) {
    match device.type_ {
        DeviceType::Eim => {
            let measurement_type = match device.measurement_type.unwrap() {
                MeasurementType::Production => "Producing",
                MeasurementType::TotalConsumption => "Consuming",
                MeasurementType::NetConsumption => "Net      ",
            };
            let details = device.details.as_ref().unwrap();
            println!(
                "{}: {:9} {:7.3} W, {} Wh so far today, {} Wh total",
                device.reading_time,
                measurement_type,
                device.w_now,
                details.wh_today,
                details.wh_lifetime,
            );
        }
        DeviceType::Inverters => {
            println!(
                "{} inverters producing {} W",
                device.active_count, device.w_now
            );
        }
        device_type => {
            println!("Unsupported device type {device_type:?}");
        }
    }
}
