// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Date

use crate::haystack::val::Value;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Date type
///
/// Holds a haystack Date value.
/// Internally uses `chrono:: NaiveDate`
///
/// # Example
/// Create a date value
/// ```
/// use chrono::Datelike;
/// use libhaystack::val::*;
///
/// let date = Value::from(Date::from_ymd(2020, 03, 22).expect("Date"));
/// assert!(date.is_date());
/// // Get the Date value
/// assert_eq!(Date::try_from(&date).unwrap().year(), 2020);
///```
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Date {
    value: NaiveDate,
}

impl Date {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Date, String> {
        let date = NaiveDate::from_ymd_opt(year, month, day);

        if let Some(value) = date {
            Ok(Date { value })
        } else {
            Err("Invalid parameters".into())
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

/// Proxy method calls to the `Date`s `value` member
impl Deref for Date {
    type Target = NaiveDate;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Proxy method calls to the mutable `Date`s `value` member
impl DerefMut for Date {
    #[inline]
    fn deref_mut(&mut self) -> &mut NaiveDate {
        &mut self.value
    }
}

/// Converts from `&str` to a `Date`
impl FromStr for Date {
    type Err = chrono::format::ParseError;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(Date {
            value: NaiveDate::from_str(val)?,
        })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

/// Converts from `NaiveDate` to a `Date`
impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self { value }
    }
}

/// Converts from `Date` to a `Date` `Value`
impl From<Date> for Value {
    fn from(value: Date) -> Self {
        Value::Date(value)
    }
}

/// Tries to convert from `Value` to a `Date`
impl TryFrom<&Value> for Date {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Date(v) => Ok(*v),
            _ => Err("Value is not an `Date`"),
        }
    }
}
