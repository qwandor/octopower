// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Functions for serialising and deserialising a timestamp as a string with `serde`.

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&dt.timestamp().to_string())
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error> {
    let s = String::deserialize(deserializer)?;
    let seconds = s.parse().map_err(|e| de::Error::custom(e))?;
    DateTime::from_timestamp(seconds, 0).ok_or_else(|| de::Error::custom(""))
}
