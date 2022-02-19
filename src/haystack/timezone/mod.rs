// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack timezone support, with fallback to UTC.
//!
//! This module can be configured to work with the full [IANA](https://www.iana.org/time-zones) database
//! provided by [chrono_tz](https://crates.io/crates/chrono-tz), or by just using a fixed offset datetime
//! by toggling the `timezone` feature.

#[cfg(feature = "timezone")]
pub mod iana;
#[cfg(feature = "timezone")]
pub(crate) use iana::*;
#[cfg(not(feature = "timezone"))]
pub mod utc;
#[cfg(not(feature = "timezone"))]
pub(crate) use utc::*;

pub(super) fn fixed_timezone(offset: &str) -> String {
    let gmt_offset = offset[2..offset.find(':').unwrap_or(3)].to_string();

    if gmt_offset == "0" {
        return "UTC".into();
    }
    let gmt_sign = offset[0..1].to_string();

    format!(
        "Etc/GMT{sign}{gmt_offset}",
        sign = if gmt_sign == "-" { "+" } else { "-" }
    )
}
