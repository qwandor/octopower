// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types used for electricity and gas consumption records, as returned by
//! [`get_consumption`](crate::get_consumption).

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A list of electricity or gas meter readings.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Readings {
    /// The total number of readings available.
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    /// The list of consumption readings in this page. This may have fewer than `count` readings.
    pub results: Vec<Consumption>,
}

/// A single consumption record from an electricity or gas meter. This may be either for a single
/// half hour or a longer grouping.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Consumption {
    /// The amount of energy consumed in this time period. For electrity meters and SMETS1 gas
    /// meters this in in kWh; for SMETS2 gas meters it is m^3.
    pub consumption: f32,
    pub interval_start: DateTime<Utc>,
    pub interval_end: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn deserialize_consumption() {
        assert_eq!(
            serde_json::from_str::<Consumption>(
                r#"{
                    "consumption": 0.42,
                    "interval_start": "2021-12-31T22:00:00Z",
                    "interval_end": "2021-12-31T22:30:00Z"
                }"#
            )
            .unwrap(),
            Consumption {
                consumption: 0.42,
                interval_start: Utc.ymd(2021, 12, 31).and_hms(22, 0, 0),
                interval_end: Utc.ymd(2021, 12, 31).and_hms(22, 30, 0)
            }
        );
    }
}
