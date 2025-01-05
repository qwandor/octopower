// Copyright 2025 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

mod config;

use config::{get_influxdb_client, Config};
use enphase_local::{
    production::{Device, DeviceType, MeasurementType, Production},
    Envoy,
};
use eyre::Report;
use influx_db_client::{Point, Precision};
use log::{debug, warn};
use tokio::time::sleep;

const INFLUXDB_PRECISION: Option<Precision> = Some(Precision::Seconds);

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let config = Config::from_file()?;
    let influxdb_client = get_influxdb_client(&config.influxdb)?;
    let envoy = Envoy::new(config.enphase.base_url, &config.enphase.token);

    loop {
        let production = envoy.production().await?;
        let points = production_to_points(&production);
        influxdb_client
            .write_points(points, INFLUXDB_PRECISION, None)
            .await?;
        sleep(config.poll_period).await;
    }
}

fn production_to_points(production: &Production) -> Vec<Point> {
    production
        .production
        .iter()
        .chain(production.consumption.iter())
        .filter_map(device_production_to_point)
        .collect()
}

fn device_production_to_point(device: &Device) -> Option<Point> {
    match device.type_ {
        DeviceType::Eim => {
            let measurement_type = match device.measurement_type.unwrap() {
                MeasurementType::Production => "Producing",
                MeasurementType::TotalConsumption => "Consuming",
                MeasurementType::NetConsumption => "Net      ",
            };
            let details = device.details.as_ref().unwrap();
            debug!(
                "{}: {:9} {:7.3} W, {} Wh so far today, {} Wh total",
                device.reading_time,
                measurement_type,
                device.w_now,
                details.wh_today,
                details.wh_lifetime,
            );
            Some(
                Point::new("eim")
                    .add_timestamp(device.reading_time.timestamp())
                    .add_tag(
                        "measurement_type",
                        tag_for_measurement_type(device.measurement_type.unwrap()),
                    )
                    .add_field("w_now", device.w_now)
                    .add_field("wh_lifetime", details.wh_lifetime),
            )
        }
        DeviceType::Inverters => {
            debug!(
                "{} inverters producing {} W",
                device.active_count, device.w_now
            );
            Some(
                Point::new("inverters")
                    .add_timestamp(device.reading_time.timestamp())
                    .add_field("w_now", device.w_now),
            )
        }
        device_type => {
            warn!("Ignoring Unsupported device type {:?}", device_type);
            None
        }
    }
}

fn tag_for_measurement_type(measurement_type: MeasurementType) -> &'static str {
    match measurement_type {
        MeasurementType::Production => "producing",
        MeasurementType::TotalConsumption => "consuming",
        MeasurementType::NetConsumption => "net",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use enphase_local::production::{AcBatteryState, Details};

    #[test]
    fn test_production_to_points() {
        let reading_time = Utc.with_ymd_and_hms(2025, 1, 1, 10, 30, 0).unwrap();
        assert_eq!(
            production_to_points(&Production {
                production: vec![
                    Device {
                        type_: DeviceType::Inverters,
                        active_count: 6,
                        measurement_type: None,
                        reading_time,
                        w_now: 123.0,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: None,
                    },
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::Production),
                        reading_time,
                        w_now: 66.0,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 4242.0,
                            ..Default::default()
                        }),
                    }
                ],
                consumption: vec![
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::TotalConsumption),
                        reading_time,
                        w_now: 61.0,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 1371.0,
                            ..Default::default()
                        })
                    },
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::NetConsumption),
                        reading_time,
                        w_now: -1.0,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 0.001,
                            ..Default::default()
                        })
                    }
                ],
                storage: vec![Device {
                    type_: DeviceType::Acb,
                    active_count: 0,
                    measurement_type: None,
                    reading_time: DateTime::from_timestamp_millis(0).unwrap(),
                    w_now: 0.0,
                    wh_now: Some(0.0),
                    state: Some(AcBatteryState::Idle),
                    details: None,
                    lines: None,
                }],
            }),
            vec![
                Point::new("inverters")
                    .add_timestamp(reading_time.timestamp())
                    .add_field("w_now", 123.0),
                Point::new("eim")
                    .add_timestamp(reading_time.timestamp())
                    .add_tag("measurement_type", "producing")
                    .add_field("w_now", 66.0)
                    .add_field("wh_lifetime", 4242.0),
                Point::new("eim")
                    .add_timestamp(reading_time.timestamp())
                    .add_tag("measurement_type", "consuming")
                    .add_field("w_now", 61.0)
                    .add_field("wh_lifetime", 1371.0),
                Point::new("eim")
                    .add_timestamp(reading_time.timestamp())
                    .add_tag("measurement_type", "net")
                    .add_field("w_now", -1.0)
                    .add_field("wh_lifetime", 0.001),
            ]
        );
    }
}
