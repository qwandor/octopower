// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the v1 production API.

use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

/// Production data for a single microinverter.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Inverter {
    /// The serial number of the microinverter.
    pub serial_number: String,
    /// The last time that the microinverter reported data.
    #[serde(with = "ts_seconds")]
    pub last_report_date: DateTime<Utc>,
    /// The device type.
    pub dev_type: u8,
    /// The power in Watts last reported.
    pub last_report_watts: u32,
    /// The maximum power in Watts which the microinverter has reported.
    pub max_report_watts: u32,
}
