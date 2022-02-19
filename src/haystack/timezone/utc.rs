// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack UTC only Timezone

use crate::timezone::fixed_timezone;
use chrono::{DateTime as StdDateTime, FixedOffset, TimeZone, Utc};

/// DateTime type that works with a fixed offset
pub(crate) type DateTimeType = StdDateTime<FixedOffset>;

pub(crate) fn make_date_time(value: DateTimeType) -> Result<DateTimeType, String> {
    Ok(value)
}

/// Constructs an UTC datetime as the timezones are not available
/// in this configuration.
pub(crate) fn make_date_time_with_tz(
    datetime: &DateTimeType,
    _tz: &str,
) -> Result<DateTimeType, String> {
    use chrono::Offset;
    Ok(Utc
        .from_utc_datetime(&datetime.naive_utc())
        .with_timezone(&Utc.fix()))
}

pub(crate) fn utc_now() -> DateTimeType {
    Utc::now().into()
}

pub(crate) fn is_utc(date: &DateTimeType) -> bool {
    use chrono::Offset;

    &Utc.fix() == date.offset()
}

pub(crate) fn timezone_short_name(date: &DateTimeType) -> String {
    let tz_id = fixed_timezone(&date.offset().to_string());

    tz_id[tz_id.find('/').map_or(0, |v| v + 1)..].to_string()
}
