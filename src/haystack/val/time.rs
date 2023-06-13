// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Time

use crate::haystack::val::Value;
use chrono::NaiveTime;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Time type
///
/// Holds a haystack Time value.
/// Internally uses `chrono:: NaiveTime`
///
/// # Example
/// Create a time value
/// ```
/// use chrono::Timelike;
/// use libhaystack::val::*;
///
/// let time = Value::from(Time::from_hms(16,20,0).expect("Time"));
/// assert!(time.is_time());
/// // Get the Time value
/// assert_eq!(Time::try_from(&time).unwrap().hour(), 16);
///```
#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Time {
    value: NaiveTime,
}

impl Time {
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> Result<Time, String> {
        let value = NaiveTime::from_hms_opt(hour, min, sec);
        if let Some(value) = value {
            Ok(Time { value })
        } else {
            Err("Invalid Time".into())
        }
    }

    pub fn from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> Result<Time, String> {
        let value = NaiveTime::from_hms_milli_opt(hour, min, sec, milli);
        if let Some(value) = value {
            Ok(Time { value })
        } else {
            Err("Invalid Time".into())
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

/// Proxy method calls to the `Time`'s `value` member
impl Deref for Time {
    type Target = NaiveTime;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Proxy method calls to the mutable `DateTime`s `value` member
impl DerefMut for Time {
    #[inline]
    fn deref_mut(&mut self) -> &mut NaiveTime {
        &mut self.value
    }
}

/// Converts from `&str` to a `Time`
impl FromStr for Time {
    type Err = chrono::format::ParseError;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(Time {
            value: NaiveTime::from_str(val)?,
        })
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

/// Converts from `NaiveTime` to a `Time`
impl From<NaiveTime> for Time {
    fn from(value: NaiveTime) -> Self {
        Self { value }
    }
}

/// Converts from `Time` to a `Time` `Value`
impl From<Time> for Value {
    fn from(value: Time) -> Self {
        Value::Time(value)
    }
}

/// Tries to convert from `Value` to a `Time`
impl TryFrom<&Value> for Time {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Time(v) => Ok(*v),
            _ => Err("Value is not an `Time`"),
        }
    }
}
