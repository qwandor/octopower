mod config;

use config::{get_influxdb_client, Config};
use eyre::Report;
use influx_db_client::{Client, Point, Precision};
use log::info;
use octopower::{account, authenticate, consumption, AuthToken, Consumption, MeterType};

const INFLUXDB_PRECISION: Option<Precision> = Some(Precision::Seconds);

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let config = Config::from_file()?;
    let influxdb_client = get_influxdb_client(&config.influxdb)?;

    let token = authenticate(&config.octopus.email_address, &config.octopus.password).await?;
    let account = account(&token, &config.octopus.account_id).await?;

    for property in &account.properties {
        info!("Property {}", property.address_line_1);
        for electricity_meter_point in &property.electricity_meter_points {
            info!("Electricity MPAN {}", electricity_meter_point.mpan);
            for meter in &electricity_meter_point.meters {
                info!("Meter serial {}", meter.serial_number);
                import_readings(
                    &token,
                    MeterType::Electricity,
                    &electricity_meter_point.mpan,
                    &meter.serial_number,
                    &influxdb_client,
                    &config.influxdb.measurement,
                    config.num_readings,
                )
                .await?;
            }
        }
        for gas_meter_point in &property.gas_meter_points {
            info!("Gas MPRN {}", gas_meter_point.mprn);
            for meter in &gas_meter_point.meters {
                info!("Meter serial {}", meter.serial_number);
                import_readings(
                    &token,
                    MeterType::Gas,
                    &gas_meter_point.mprn,
                    &meter.serial_number,
                    &influxdb_client,
                    &config.influxdb.measurement,
                    config.num_readings,
                )
                .await?;
            }
        }
    }

    Ok(())
}

async fn import_readings(
    token: &AuthToken,
    meter_type: MeterType,
    mpxn: &str,
    serial: &str,
    influxdb_client: &Client,
    measurement: &str,
    num_readings: usize,
) -> Result<(), Report> {
    let consumption = consumption(&token, meter_type, mpxn, serial, 1, num_readings, None).await?;
    info!(
        "{:?} consumption: {}/{} records",
        meter_type,
        consumption.results.len(),
        consumption.count
    );
    let points = consumption
        .results
        .into_iter()
        .map(|reading| point_for_reading(measurement, meter_type, mpxn, serial, reading));
    influxdb_client
        .write_points(points, INFLUXDB_PRECISION, None)
        .await?;

    Ok(())
}

fn point_for_reading(
    measurement: &str,
    meter_type: MeterType,
    mpxn: &str,
    serial: &str,
    reading: Consumption,
) -> Point {
    Point::new(measurement)
        .add_timestamp(reading.interval_end.timestamp())
        .add_tag("type", meter_type.to_string())
        .add_tag("mpxn", mpxn)
        .add_tag("serial", serial)
        .add_field("consumption", reading.consumption as f64)
}
