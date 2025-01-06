// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A list of electricity meter standing charges.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct StandingUnitRates {
    /// The total number of unit rates
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    /// The list of unit rate readings in this page. This may have fewer than 'count' readings.
    pub results: Vec<StandingUnitRate>,
}

/// A single unit rate record from an electricity or gas meter. This may be either for a single half hour
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct StandingUnitRate {
    // $ in pence
    pub value_exc_vat: f32,
    pub value_inc_vat: f32,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub payment_method: Option<String>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn deserialize_consumption() {
        assert_eq!(
            serde_json::from_str::<StandingUnitRate>(
                r#"{
                        "value_exc_vat": 4.62,
                        "value_inc_vat": 4.851,
                        "valid_from": "2023-12-29T22:30:00Z",
                        "valid_to": "2023-12-29T23:00:00Z",
                        "payment_method": null
                    }"#
            )
            .unwrap(),
            StandingUnitRate {
                value_exc_vat: 4.62,
                value_inc_vat: 4.851,
                valid_from: Utc
                    .with_ymd_and_hms(2023, 12, 29, 22, 30, 0)
                    .single()
                    .unwrap(),
                valid_to: Utc
                    .with_ymd_and_hms(2023, 12, 29, 23, 00, 0)
                    .single()
                    .unwrap(),
                payment_method: None
            }
        );
    }
}
