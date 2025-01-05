// Copyright 2025 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types returned by the IVP meters API methods.

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::production::MeasurementType;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reading {
    /// A summary of all channels.
    #[serde(flatten)]
    summary: Channel,
    /// The individual channels (typically different phases) that make up this reading.
    channels: Vec<Channel>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// Gateway record ID number.
    eid: u64,
    /// Time when the message was generated.
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
    /// Active energy delivered.
    act_energy_dlvd: f64,
    /// Active energy received.
    act_energy_rcvd: f64,
    /// Apparent energy.
    apparent_energy: f64,
    /// Lagging reactive energy.
    react_energy_lagg: f64,
    /// Leading reactive energy.
    react_energy_lead: f64,
    /// Instantaneous demand.
    instantaneous_demand: f64,
    /// Active power.
    active_power: f64,
    /// Apparent power.
    apparent_power: f64,
    /// Reactive power.
    reactive_power: f64,
    /// Power factor.
    pwr_factor: f64,
    /// Voltage.
    voltage: f64,
    /// Current.
    current: f64,
    /// Frequency.
    freq: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// Time when the message was generated.
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    report_type: MeasurementType,
    cumulative: MeterReading,
    lines: Vec<MeterReading>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterReading {
    /// Current power in Watts.
    curr_w: f64,
    /// Active power.
    act_power: f64,
    /// Apparent power.
    apprnt_pwr: f64,
    /// Reactive power.
    react_pwr: f64,
    /// Cumulative Watt-hours delivered.
    wh_dlvd_cum: f64,
    /// Cumulative Watt-hours received.
    wh_rcvd_cum: f64,
    /// Cumulative lagging varh.
    varh_lag_cum: f64,
    /// Cumulative leading varh.
    varh_lead_cum: f64,
    /// Cumulative vah.
    vah_cum: f64,
    /// RMS voltage.
    rms_voltage: f64,
    /// RMS current.
    rms_current: f64,
    /// Power factor.
    pwr_factor: f64,
    /// Frequency in Hertz.
    freq_hz: f64,
}
