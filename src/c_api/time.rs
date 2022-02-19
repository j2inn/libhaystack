// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Time](crate::val::Time) type.
//!

use chrono::Timelike;

use super::err::new_error;
use crate::haystack::val::Value;

/// Get the hour of a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Arguments
/// val A [Time](crate::val::Time) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::time::*;
/// # unsafe {
/// let val = haystack_value_make_time(12, 12, 59);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_time(val));
/// assert_eq!(haystack_value_get_time_hour(val), 12);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_time_hour(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Time(time) => return time.hour(),
            _ => new_error("Not a Time Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}

/// Get the minutes of a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Arguments
/// val A [Time](crate::val::Time) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::time::*;
/// # unsafe {
/// let val = haystack_value_make_time(12, 15, 59);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_time(val));
/// assert_eq!(haystack_value_get_time_minutes(val), 15);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_time_minutes(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Time(time) => return time.minute(),
            _ => new_error("Not a Time Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}

/// Get the seconds of a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Arguments
/// val A [Time](crate::val::Time) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::time::*;
/// # unsafe {
/// let val = haystack_value_make_time(12, 15, 59);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_time(val));
/// assert_eq!(haystack_value_get_time_seconds(val), 59);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_time_seconds(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Time(time) => return time.second(),
            _ => new_error("Not a Time Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}

/// Get the milliseconds of a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Arguments
/// val A [Time](crate::val::Time) [Value](crate::val::Value)
/// # Returns
/// - value
/// - -1 (u32::MAX) if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::time::*;
/// # unsafe {
/// let val = haystack_value_make_time_millis(12, 15, 59, 999);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_time(val));
/// assert_eq!(haystack_value_get_time_millis(val), 999);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_time_millis(val: *const Value) -> u32 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Time(time) => return time.nanosecond() / 1_000_000,
            _ => new_error("Not a Time Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    u32::MAX
}
