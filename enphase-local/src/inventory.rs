// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the inventory API.

use crate::timestamp_string;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The inventory of all devices.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inventory(pub Vec<InventoryGroup>);

/// A group of devices of a particular type in the inventory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InventoryGroup {
    #[serde(rename = "type")]
    pub type_: DeviceType,
    pub devices: Vec<Device>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    /// Power Conditioning Unit, aka. microinverter.
    Pcu,
    /// AC Battery.
    Acb,
    /// Network System Relay Breaker, aka. IQ Relay.
    Nsrb,
    /// Electrical sub-panel, aka. IQ System Controller.
    Esub,
}

/// The state of a device in the inventory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Device {
    /// The part number of the device.
    pub part_num: String,
    /// When the device was installed.
    #[serde(with = "timestamp_string")]
    pub installed: DateTime<Utc>,
    /// The serial number of the device.
    pub serial_num: String,
    /// The device's current statuses.
    pub device_status: Vec<DeviceStatus>,
    /// When the device last reported to the gateway.
    #[serde(with = "timestamp_string")]
    pub last_rpt_date: DateTime<Utc>,
    /// The administrative state of the device.
    pub admin_state: AdminState,
    /// The type of device.
    pub dev_type: u8,
    /// When the device was added to the gateway.
    #[serde(with = "timestamp_string")]
    pub created_date: DateTime<Utc>,
    /// When the device's firmware was loaded.
    #[serde(with = "timestamp_string")]
    pub img_load_date: DateTime<Utc>,
    /// The device's firmware product number.
    pub img_pnum_running: String,
    pub ptpn: String,
    /// The channel Enphase ID.
    pub chaneid: u64,
    pub device_control: Vec<DeviceControl>,
    /// Whether the device is producing electricity.
    pub producing: bool,
    /// Whether the device is communicating with the gateway.
    pub communicating: bool,
    /// Whether the device is provisioned.
    pub provisioned: bool,
    /// Whether the device is operating.
    pub operating: bool,
    pub phase: String,
}

/// The status flags of a device.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DeviceStatus {
    #[serde(rename = "envoy.global.ok")]
    Ok,
    #[serde(rename = "envoy.cond_flags.pcu_chan.dcvoltagetoolow")]
    DcVoltageTooLow,
    #[serde(rename = "envoy.cond_flags.pcu_ctrl.dc-pwr-low")]
    DcPowerLow,
    #[serde(rename = "envoy.cond_flags.obs_strs.failure")]
    Failure,
}

/// The administrative state of a device.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum AdminState {
    Discovered = 1,
    Verified = 2,
    Deleted = 3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeviceControl {
    /// Whether the device has a Ground Fault Interrupt error state
    pub gficlearset: bool,
}
