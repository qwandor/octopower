// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the production API.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            serde_json::from_str::<Production>(
                r#"{
    "production": [
        {
            "type": "inverters",
            "activeCount": 10,
            "readingTime": 1672574917,
            "wNow": 225,
            "whLifetime": 22444
        },
        {
            "type": "eim",
            "activeCount": 0,
            "measurementType": "production",
            "readingTime": 1672575063,
            "wNow": 63.302,
            "whLifetime": 1513.493,
            "varhLeadLifetime": 0.024,
            "varhLagLifetime": 16902.01,
            "vahLifetime": 23774.477,
            "rmsCurrent": 2.154,
            "rmsVoltage": 240.087,
            "reactPwr": 453.423,
            "apprntPwr": 517.896,
            "pwrFactor": 0.13,
            "whToday": 2.0,
            "whLastSevenDays": 1520.0,
            "vahToday": 5106.0,
            "varhLeadToday": 0.0,
            "varhLagToday": 3865.0
        }
    ],
    "consumption": [
        {
            "type": "eim",
            "activeCount": 0,
            "measurementType": "total-consumption",
            "readingTime": 1672575063,
            "wNow": 61.709,
            "whLifetime": 1371.426,
            "varhLeadLifetime": 0.205,
            "varhLagLifetime": 16918.508,
            "vahLifetime": 2593.65,
            "rmsCurrent": 1.792,
            "rmsVoltage": 243.568,
            "reactPwr": -452.024,
            "apprntPwr": 436.397,
            "pwrFactor": 0.14,
            "whToday": 0.0,
            "whLastSevenDays": 1465.0,
            "vahToday": 695.65,
            "varhLeadToday": 0.205,
            "varhLagToday": 3875.508
        },
        {
            "type": "eim",
            "activeCount": 0,
            "measurementType": "net-consumption",
            "readingTime": 1672575063,
            "wNow": -1.592,
            "whLifetime": 0.001,
            "varhLeadLifetime": 0.181,
            "varhLagLifetime": 16.498,
            "vahLifetime": 2593.65,
            "rmsCurrent": 0.363,
            "rmsVoltage": 247.049,
            "reactPwr": 1.398,
            "apprntPwr": 61.047,
            "pwrFactor": 0.0,
            "whToday": 0,
            "whLastSevenDays": 0,
            "vahToday": 0,
            "varhLeadToday": 0,
            "varhLagToday": 0
        }
    ],
    "storage": [
        {
            "type": "acb",
            "activeCount": 0,
            "readingTime": 0,
            "wNow": 0,
            "whNow": 0,
            "state": "idle"
        }
    ]
}
            "#,
            )
            .unwrap(),
            Production {
                production: vec![
                    Device {
                        type_: DeviceType::Inverters,
                        active_count: 10,
                        measurement_type: None,
                        reading_time: "2023-01-01T12:08:37Z".parse().unwrap(),
                        w_now: 225.0,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: None,
                    },
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::Production),
                        reading_time: "2023-01-01T12:11:03Z".parse().unwrap(),
                        w_now: 63.302,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 1513.493,
                            varh_lead_lifetime: 0.024,
                            varh_lag_lifetime: 16902.01,
                            vah_lifetime: 23774.477,
                            rms_current: 2.154,
                            rms_voltage: 240.087,
                            react_pwr: 453.423,
                            apprnt_pwr: 517.896,
                            pwr_factor: 0.13,
                            wh_today: 2.0,
                            wh_last_seven_days: 1520.0,
                            vah_today: 5106.0,
                            varh_lead_today: 0.0,
                            varh_lag_today: 3865.0
                        })
                    }
                ],
                consumption: vec![
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::TotalConsumption),
                        reading_time: "2023-01-01T12:11:03Z".parse().unwrap(),
                        w_now: 61.709,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 1371.426,
                            varh_lead_lifetime: 0.205,
                            varh_lag_lifetime: 16918.508,
                            vah_lifetime: 2593.65,
                            rms_current: 1.792,
                            rms_voltage: 243.568,
                            react_pwr: -452.024,
                            apprnt_pwr: 436.397,
                            pwr_factor: 0.14,
                            wh_today: 0.0,
                            wh_last_seven_days: 1465.0,
                            vah_today: 695.65,
                            varh_lead_today: 0.205,
                            varh_lag_today: 3875.508
                        })
                    },
                    Device {
                        type_: DeviceType::Eim,
                        active_count: 0,
                        measurement_type: Some(MeasurementType::NetConsumption),
                        reading_time: "2023-01-01T12:11:03Z".parse().unwrap(),
                        w_now: -1.592,
                        wh_now: None,
                        state: None,
                        lines: None,
                        details: Some(Details {
                            wh_lifetime: 0.001,
                            varh_lead_lifetime: 0.181,
                            varh_lag_lifetime: 16.498,
                            vah_lifetime: 2593.65,
                            rms_current: 0.363,
                            rms_voltage: 247.049,
                            react_pwr: 1.398,
                            apprnt_pwr: 61.047,
                            pwr_factor: 0.0,
                            wh_today: 0.0,
                            wh_last_seven_days: 0.0,
                            vah_today: 0.0,
                            varh_lead_today: 0.0,
                            varh_lag_today: 0.0
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
            }
        );
    }
}
