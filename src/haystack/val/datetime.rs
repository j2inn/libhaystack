// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack DateTime

use crate::haystack::val::Value;
use crate::timezone::{
    is_utc, make_date_time, make_date_time_with_tz, timezone_short_name, utc_now, DateTimeType,
};
use chrono::{DateTime as DateTimeImpl, FixedOffset, Utc};

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// DateTime
///
/// Holds a haystack DateTime value.
/// Internally uses `chrono:: DateTime<Tz>`
///
/// # Example
/// Create a datetime value
/// ```
/// use chrono::Timelike;
/// use libhaystack::val::*;
///
/// let datetime = Value::make_datetime_from_iso("1996-12-19T16:39:57-08:00").unwrap();
/// assert!(datetime.is_datetime());
/// // Get the DateTime value
/// assert_eq!(DateTime::try_from(&datetime).unwrap().hour(), 16);
///```
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct DateTime {
    value: DateTimeType,
}

impl DateTime {
    pub fn parse_from_rfc3339(arg: &str) -> Result<DateTime, String> {
        match DateTimeImpl::<FixedOffset>::parse_from_rfc3339(arg) {
            Ok(value) => Ok(DateTime {
                value: make_date_time(value)?,
            }),
            Err(err) => Err(format!("Can't parse date time {err}")),
        }
    }

    /// Construct a `DateTime` from an ISO8601 and a Timezone
    pub fn parse_from_rfc3339_with_timezone(datetime: &str, tz: &str) -> Result<DateTime, String> {
        match DateTimeImpl::<FixedOffset>::parse_from_rfc3339(datetime) {
            Ok(value) => Ok(DateTime {
                value: make_date_time_with_tz(&value, tz)?,
            }),
            Err(err) => Err(format!("Can't parse date time {err}")),
        }
    }

    pub fn utc_now() -> Self {
        DateTime { value: utc_now() }
    }

    /// True is the timezone of this date is UTC
    pub fn is_utc(&self) -> bool {
        is_utc(&self.value)
    }

    /// Get this `DateTime`'s short variant timezone name, or the city name of the timezone
    pub fn timezone_short_name(&self) -> String {
        timezone_short_name(&self.value)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

/// Proxy method calls to the `DateTime`'s `value` member
impl Deref for DateTime {
    type Target = DateTimeType;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Proxy method calls to the mutable `DateTime`s `value` member
impl DerefMut for DateTime {
    #[inline]
    fn deref_mut(&mut self) -> &mut DateTimeType {
        &mut self.value
    }
}

/// Converts from `&str` to a `Date`
impl FromStr for DateTime {
    type Err = String;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_rfc3339(val)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

/// Converts from `DateTime` to a `DateTime` `Value`
impl From<DateTime> for Value {
    fn from(value: DateTime) -> Self {
        Value::DateTime(value)
    }
}

/// Converts from `DateTimeImpl<Utc>` to a `DateTime` `Value`
#[cfg(feature = "timezone")]
impl From<DateTimeImpl<Utc>> for Value {
    fn from(from: DateTimeImpl<Utc>) -> Self {
        Value::DateTime(DateTime::from(from))
    }
}

/// Converts from `DateTimeImpl<Utc>` to a `DateTime` `Value`
#[cfg(not(feature = "timezone"))]
impl From<DateTimeImpl<Utc>> for Value {
    fn from(from: DateTimeImpl<Utc>) -> Self {
        Value::DateTime(DateTime { value: from.into() })
    }
}

/// Converts from `DateTimeImpl<Utc>` to a `DateTime`
#[cfg(not(feature = "timezone"))]
impl From<DateTimeImpl<Utc>> for DateTime {
    fn from(from: DateTimeImpl<Utc>) -> Self {
        DateTime { value: from.into() }
    }
}

/// Converts from `DateTimeImpl<FixedOffset>` to a `DateTime`
#[cfg(not(feature = "timezone"))]
impl From<DateTimeImpl<FixedOffset>> for DateTime {
    fn from(from: DateTimeImpl<FixedOffset>) -> Self {
        DateTime { value: from }
    }
}

/// Converts from `DateTimeImpl<Utc>` to a `DateTime`
#[cfg(feature = "timezone")]
impl From<DateTimeImpl<Utc>> for DateTime {
    fn from(from: DateTimeImpl<Utc>) -> Self {
        DateTime {
            value: from.with_timezone(&chrono_tz::UTC),
        }
    }
}

/// Converts from `DateTimeImpl<Tz>` to a `DateTime`
#[cfg(feature = "timezone")]
impl From<DateTimeImpl<chrono_tz::Tz>> for DateTime {
    fn from(value: DateTimeImpl<chrono_tz::Tz>) -> Self {
        DateTime { value }
    }
}

/// Tries to convert from `Value` to a `DateTime`
impl TryFrom<&Value> for DateTime {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(v) => Ok(*v),
            _ => Err("Value is not an `DateTime`"),
        }
    }
}
