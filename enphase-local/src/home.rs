// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the home API.

use chrono::{DateTime, NaiveTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Home {
    /// When the software was built.
    #[serde(with = "ts_seconds")]
    pub software_build_epoch: DateTime<Utc>,
    /// The local timezone configured for the gateway.
    pub timezone: String,
    /// The current date according to the gateway.
    pub current_date: String,
    /// The current time according to the gateway.
    pub current_time: NaiveTime,
    /// Information about the network interfaces of the gatewway.
    pub network: Network,
    /// The type of tariff configured.
    pub tariff: String,
    /// Information about communications with devices.
    pub comm: CommunicationSummary,
    /// Information about wireless connections.
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
pub struct CommunicationSummary {
    /// The number of devices communicating.
    pub num: u8,
    /// The signal level.
    pub level: u8,
    /// Information about Power Conditioning Units.
    pub pcu: CommunicationStatus,
    /// Information about AC Batteries.
    pub acb: CommunicationStatus,
    /// Information about Network System Relay Breakers.
    pub nsrb: CommunicationStatus,
    /// Information about Electrical Sub-panels.
    pub esub: CommunicationStatus,
    /// Information about IQ Batteries (aka. Encharge Storage).
    pub encharge: Vec<EnchargeCommunicationStatus>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommunicationStatus {
    /// The number of devices communicating.
    pub num: u8,
    /// The signal level.
    pub level: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnchargeCommunicationStatus {
    /// The number of devices communicating.
    pub num: u8,
    /// The signal level.
    pub level: u8,
    /// The 2.4 GHz signal level.
    pub level_24g: u8,
    /// The sub-GHz signal level.
    pub level_subg: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WirelessConnection {
    /// The current signal strength.
    pub signal_strength: u8,
    /// The maximum signal strength recorded.
    pub signal_strength_max: u8,
    /// The type of wireless connection.
    #[serde(rename = "type")]
    pub type_: WirelessConnectionType,
    /// Whether the connection is connected.
    pub connected: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum WirelessConnectionType {
    #[serde(rename = "BLE")]
    Ble,
}
