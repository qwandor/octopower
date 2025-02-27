// Copyright 2025 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

mod config;

use config::{get_influxdb_client, Config};
use enphase_local::{
    inverters::Inverter,
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

    let mut last_inverters = Vec::new();
    loop {
        let production = envoy.production().await?;
        let points = production_to_points(&production);
        influxdb_client
            .write_points(points, INFLUXDB_PRECISION, None)
            .await?;

        let inverters = envoy.inverters().await?;
        let points = inverters_to_points(&inverters, &last_inverters);
        if !points.is_empty() {
            influxdb_client
                .write_points(points, INFLUXDB_PRECISION, None)
                .await?;
        }
        last_inverters = inverters;

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
            let Some(measurement_type) = device.measurement_type else {
                warn!("EIM device missing measurement type: {:?}", device);
                return None;
            };
            let Some(details) = device.details.as_ref() else {
                warn!("EIM device missing details: {:?}", device);
                return None;
            };
            let debug_measurement_type = match measurement_type {
                MeasurementType::Production => "Producing",
                MeasurementType::TotalConsumption => "Consuming",
                MeasurementType::NetConsumption => "Net      ",
            };
            debug!(
                "{}: {:9} {:7.3} W, {} Wh so far today, {} Wh total",
                device.reading_time,
                debug_measurement_type,
                device.w_now,
                details.wh_today,
                details.wh_lifetime,
            );
            Some(
                Point::new("eim")
                    .add_timestamp(device.reading_time.timestamp())
                    .add_tag(
                        "measurement_type",
                        tag_for_measurement_type(measurement_type),
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

fn inverters_to_points<'a>(
    inverters: &'a [Inverter],
    last_inverters: &[Inverter],
) -> Vec<Point<'a>> {
    inverters
        .iter()
        .filter(|inverter| {
            // Don't emit a point if the inverter reading hasn't changed since last time.
            Some(*inverter)
                != last_inverters
                    .iter()
                    .find(|last_inverter| last_inverter.serial_number == inverter.serial_number)
        })
        .map(inverter_to_point)
        .collect()
}

fn inverter_to_point(inverter: &Inverter) -> Point {
    debug!(
        "{} Inverter {} producing {} W (max {} W)",
        inverter.last_report_date,
        inverter.serial_number,
        inverter.last_report_watts,
        inverter.max_report_watts,
    );
    Point::new("inverter")
        .add_timestamp(inverter.last_report_date.timestamp())
        .add_tag("serial_number", inverter.serial_number.as_str())
        .add_field("last_watts", i64::from(inverter.last_report_watts))
        .add_field("max_watts", i64::from(inverter.max_report_watts))
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

    #[test]
    fn test_inverters_to_points() {
        let last_report_date = Utc.with_ymd_and_hms(2025, 1, 1, 10, 30, 0).unwrap();
        let last_report_date_old = Utc.with_ymd_and_hms(2025, 1, 1, 2, 30, 0).unwrap();
        assert_eq!(inverters_to_points(&[], &[]), vec![]);
        let inverter1 = Inverter {
            last_report_date,
            dev_type: 1,
            serial_number: "1".to_string(),
            last_report_watts: 42,
            max_report_watts: 66,
        };
        let inverter2_old = Inverter {
            last_report_date: last_report_date_old,
            dev_type: 1,
            serial_number: "2".to_string(),
            last_report_watts: 22,
            max_report_watts: 600,
        };
        let inverter2 = Inverter {
            last_report_date,
            dev_type: 1,
            serial_number: "2".to_string(),
            last_report_watts: 33,
            max_report_watts: 600,
        };
        assert_eq!(
            inverters_to_points(&[inverter1.clone()], &[]),
            vec![Point::new("inverter")
                .add_timestamp(last_report_date.timestamp())
                .add_tag("serial_number", "1")
                .add_field("last_watts", 42)
                .add_field("max_watts", 66)]
        );
        assert_eq!(
            inverters_to_points(&[inverter1.clone(), inverter2.clone()], &[]),
            vec![
                Point::new("inverter")
                    .add_timestamp(last_report_date.timestamp())
                    .add_tag("serial_number", "1")
                    .add_field("last_watts", 42)
                    .add_field("max_watts", 66),
                Point::new("inverter")
                    .add_timestamp(last_report_date.timestamp())
                    .add_tag("serial_number", "2")
                    .add_field("last_watts", 33)
                    .add_field("max_watts", 600),
            ]
        );
        assert_eq!(
            inverters_to_points(&[inverter1.clone(), inverter2], &[inverter1, inverter2_old]),
            vec![Point::new("inverter")
                .add_timestamp(last_report_date.timestamp())
                .add_tag("serial_number", "2")
                .add_field("last_watts", 33)
                .add_field("max_watts", 600)]
        );
    }
}
