// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the home API.

use chrono::{serde::ts_seconds, DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Home {
    #[serde(with = "ts_seconds")]
    pub software_build_epoch: DateTime<Utc>,
    pub timezone: String,
    pub current_date: String,
    pub current_time: NaiveTime,
    pub network: Network,
    pub tariff: String,
    pub comm: Comm,
    pub wireless_connection: Vec<WirelessConnection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Network {
    pub web_comm: bool,
    pub ever_reported_to_enlighten: bool,
    #[serde(with = "ts_seconds")]
    pub last_enlighten_report_time: DateTime<Utc>,
    pub primary_interface: String,
    pub interfaces: Vec<Interface>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Interface {
    #[serde(rename = "type")]
    pub type_: InterfaceType,
    pub mac: String,
    pub dhcp: bool,
    pub ip: IpAddr,
    pub signal_strength: u8,
    pub signal_strength_max: u8,
    pub carrier: bool,
    pub supported: Option<bool>,
    pub present: Option<bool>,
    pub configured: Option<bool>,
    pub status: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceType {
    Ethernet,
    Wifi,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Comm {
    num: u8,
    level: u8,
    encharge: Vec<Encharge>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Encharge {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WirelessConnection {
    pub signal_strength: u8,
    pub signal_strength_max: u8,
    #[serde(rename = "type")]
    pub type_: WirelessConnectionType,
    pub connected: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WirelessConnectionType {
    #[serde(rename = "BLE")]
    Ble,
}
