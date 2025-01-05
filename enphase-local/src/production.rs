// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the production API.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Production {
    pub production: Vec<Device>,
    pub consumption: Vec<Device>,
    pub storage: Vec<Device>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// The type of device this is about.
    #[serde(rename = "type")]
    pub type_: DeviceType,
    /// The number of active devices of this type.
    pub active_count: u32,
    pub measurement_type: Option<MeasurementType>,
    /// The time at which the reading was taken.
    #[serde(with = "ts_seconds")]
    pub reading_time: DateTime<Utc>,
    /// The power in watts currently measured.
    pub w_now: f64,
    pub wh_now: Option<f64>,
    pub state: Option<AcBatteryState>,
    pub lines: Option<Vec<Line>>,
    #[serde(flatten)]
    pub details: Option<Details>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    /// The power in watts currently measured.
    pub w_now: f64,
    #[serde(flatten)]
    pub details: Details,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// The number of watt-hours this device has measured.
    pub wh_lifetime: f64,
    /// The total leading volt-amp reactive hours this device has measured.
    pub varh_lead_lifetime: f64,
    /// The total lagging volt-amp reactive hours this device has measured.
    pub varh_lag_lifetime: f64,
    /// The total volt-amp hours this device has measured.
    pub vah_lifetime: f64,
    /// The root-mean-square current currently measured.
    pub rms_current: f64,
    /// The root-mean-square voltage currently measured.
    pub rms_voltage: f64,
    pub react_pwr: f64,
    pub apprnt_pwr: f64,
    pub pwr_factor: f64,
    pub wh_today: f64,
    pub wh_last_seven_days: f64,
    pub vah_today: f64,
    pub varh_lead_today: f64,
    pub varh_lag_today: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// IQ Inverters.
    Inverters,
    /// The Envoy Integrated Meter built-in to the gateway.
    Eim,
    /// AC battery.
    Acb,
    /// Revenue Grade Meters.
    Rgms,
    /// Power Meter Units.
    Pmus,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeasurementType {
    /// Energy producted from the inverters.
    Production,
    /// Load minus solar production.
    NetConsumption,
    /// Total load.
    TotalConsumption,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AcBatteryState {
    Charging,
    Discharging,
    Full,
    Idle,
}
