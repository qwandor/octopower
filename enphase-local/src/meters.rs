// Copyright 2025 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the IVP meters API methods.

use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

use crate::production::MeasurementType;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reading {
    /// A summary of all channels.
    #[serde(flatten)]
    pub summary: Channel,
    /// The individual channels (typically different phases) that make up this reading.
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// Gateway record ID number.
    pub eid: u64,
    /// Time when the message was generated.
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    /// Active energy delivered.
    pub act_energy_dlvd: f64,
    /// Active energy received.
    pub act_energy_rcvd: f64,
    /// Apparent energy.
    pub apparent_energy: f64,
    /// Lagging reactive energy.
    pub react_energy_lagg: f64,
    /// Leading reactive energy.
    pub react_energy_lead: f64,
    /// Instantaneous demand.
    pub instantaneous_demand: f64,
    /// Active power.
    pub active_power: f64,
    /// Apparent power.
    pub apparent_power: f64,
    /// Reactive power.
    pub reactive_power: f64,
    /// Power factor.
    pub pwr_factor: f64,
    /// Voltage.
    pub voltage: f64,
    /// Current.
    pub current: f64,
    /// Frequency.
    pub freq: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// Time when the message was generated.
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub report_type: MeasurementType,
    pub cumulative: MeterReading,
    pub lines: Vec<MeterReading>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterReading {
    /// Current power in Watts.
    pub curr_w: f64,
    /// Active power.
    pub act_power: f64,
    /// Apparent power.
    pub apprnt_pwr: f64,
    /// Reactive power.
    pub react_pwr: f64,
    /// Cumulative Watt-hours delivered.
    pub wh_dlvd_cum: f64,
    /// Cumulative Watt-hours received.
    pub wh_rcvd_cum: f64,
    /// Cumulative lagging varh.
    pub varh_lag_cum: f64,
    /// Cumulative leading varh.
    pub varh_lead_cum: f64,
    /// Cumulative vah.
    pub vah_cum: f64,
    /// RMS voltage.
    pub rms_voltage: f64,
    /// RMS current.
    pub rms_current: f64,
    /// Power factor.
    pub pwr_factor: f64,
    /// Frequency in Hertz.
    pub freq_hz: f64,
}
