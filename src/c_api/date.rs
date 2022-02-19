// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Date](crate::val::Date) type.
//!

use chrono::Datelike;

use super::err::new_error;
use crate::haystack::val::Value;

/// Get the year of a [Date](crate::val::Date) [Value](crate::val::Value)
/// # Arguments
/// val A [Date](crate::val::Date) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::date::*;
/// # unsafe {
/// let val = haystack_value_make_date(2021, 8, 13);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_date(val));
/// assert_eq!(haystack_value_get_date_year(val), 2021);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_date_year(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Date(date) => return date.year() as u32,
            _ => new_error("Not a Date Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}

/// Get the month of a [Date](crate::val::Date) [Value](crate::val::Value)
/// # Arguments
/// val A [Date](crate::val::Date) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::date::*;
/// # unsafe {
/// let val = haystack_value_make_date(2021, 8, 13);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_date(val));
/// assert_eq!(haystack_value_get_date_month(val), 8);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_date_month(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Date(date) => return date.month(),
            _ => new_error("Not a Date Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}

/// Get the day of a [Date](crate::val::Date) [Value](crate::val::Value)
/// # Arguments
/// val A [Date](crate::val::Date) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::date::*;
/// # unsafe {
/// let val = haystack_value_make_date(2021, 8, 13);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_date(val));
/// assert_eq!(haystack_value_get_date_day(val), 13);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_date_day(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Date(date) => return date.day(),
            _ => new_error("Not a Date Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}
