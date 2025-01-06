// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types used for account information, as returned by [`get_account`](crate::get_account).

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

/// Information about an Octopus account.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Account {
    /// The account number. This is usually of the form `"A-1234ABCD"`.
    pub number: String,
    /// The properties associated with this account.
    pub properties: Vec<Property>,
}

/// Information about a particular property within an Octopus account.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Property {
    pub id: u32,
    pub moved_in_at: DateTime<FixedOffset>,
    pub moved_out_at: Option<DateTime<FixedOffset>>,
    pub address_line_1: String,
    pub address_line_2: String,
    pub address_line_3: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub electricity_meter_points: Vec<ElectricityMeterPoint>,
    pub gas_meter_points: Vec<GasMeterPoint>,
}

/// Information about a particular electricity meter point at a property. This may include several
/// different meters.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ElectricityMeterPoint {
    pub mpan: String,
    pub profile_class: u32,
    pub consumption_standard: u32,
    /// The electricity meters included in this meter point.
    pub meters: Vec<Meter>,
    pub agreements: Vec<Agreement>,
    pub is_export: bool,
}

/// Information about a particular gas meter point at a property. This may include several different
/// meters.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GasMeterPoint {
    pub mprn: String,
    pub consumption_standard: u32,
    /// The gas meters included in this meter point.
    pub meters: Vec<Meter>,
    pub agreements: Vec<Agreement>,
}

/// Information about a single electricity or gas meter at a property.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Meter {
    pub serial_number: String,
    #[serde(default)]
    pub registers: Vec<Register>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Register {
    pub identifier: String,
    pub rate: String,
    pub is_settlement_register: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Agreement {
    pub tariff_code: String,
    pub valid_from: DateTime<FixedOffset>,
    pub valid_to: Option<DateTime<FixedOffset>>,
}
