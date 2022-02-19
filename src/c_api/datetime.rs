// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [DateTime](crate::val::DateTime) type.
//!

use super::err::{new_error, update_last_error};
use super::ResultType;
use crate::haystack::val::{Date, Time, Value};
use std::ffi::CString;
use std::os::raw::c_char;

/// Get the date of a [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Arguments
/// - val A [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// - utc True if need to return the utc variant, false returns the local variant
/// - result The Value to be updated with the result DateTime.
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::datetime::*;
/// # unsafe {
/// let date = haystack_value_make_date(2021, 8, 13);
/// let time = haystack_value_make_time(9, 45, 59);
/// # let date = Box::<Value>::leak(date.unwrap());
/// # let time = Box::<Value>::leak(time.unwrap());
/// let tz = std::ffi::CString::new("New_York").unwrap();
/// let val = haystack_value_make_tz_datetime(date, time, tz.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_datetime(val));
/// # let result = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_value_get_datetime_date(val, true, result), ResultType::TRUE);
/// # assert_eq!((*result).to_string(), (*date).to_string());
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_datetime_date(
    val: *const Value,
    utc: bool,
    result: *mut Value,
) -> ResultType {
    if val.is_null() || result.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }
    match val.as_ref() {
        Some(value) => match value {
            Value::DateTime(datetime) => {
                let date = if utc {
                    datetime.naive_utc().date()
                } else {
                    datetime.naive_local().date()
                };
                if let Some(value) = result.as_mut() {
                    *value = Date::from(date).into();
                    return ResultType::TRUE;
                } else {
                    new_error("Not a Value result.");
                }
            }
            _ => new_error("Not a DateTime Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Get the time of a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Arguments
/// - val A [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// - utc True if need to return the utc variant, false returns the local variant
/// - result The Value to be updated with the result Time.
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::datetime::*;
/// # unsafe {
/// let date = haystack_value_make_date(2021, 8, 13);
/// let time = haystack_value_make_time(9, 45, 59);
/// # let date = Box::<Value>::leak(date.unwrap());
/// # let time = Box::<Value>::leak(time.unwrap());
/// let tz = std::ffi::CString::new("New_York").unwrap();
/// let val = haystack_value_make_tz_datetime(date, time, tz.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_datetime(val));
/// # let result = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_value_get_datetime_time(val, true, result), ResultType::TRUE);
/// # assert_eq!((*result).to_string(), (*time).to_string());
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_datetime_time(
    val: *const Value,
    utc: bool,
    result: *mut Value,
) -> ResultType {
    match val.as_ref() {
        Some(value) => match value {
            Value::DateTime(datetime) => {
                let time = if utc {
                    datetime.naive_utc().time()
                } else {
                    datetime.naive_local().time()
                };
                if let Some(value) = result.as_mut() {
                    *value = Time::from(time).into();
                    return ResultType::TRUE;
                } else {
                    new_error("Not a Value result.");
                }
            }
            _ => new_error("Not a DateTime Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Get the timezone of a [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Arguments
/// - val A [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Returns
/// The timezone name as a C String or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::datetime::*;
/// # unsafe {
/// let date = haystack_value_make_date(2021, 8, 13);
/// let time = haystack_value_make_time(9, 45, 59);
/// # let date = Box::<Value>::leak(date.unwrap());
/// # let time = Box::<Value>::leak(time.unwrap());
/// let tz = std::ffi::CString::new("New_York").unwrap();
/// let val = haystack_value_make_tz_datetime(date, time, tz.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_datetime(val));
/// let res = haystack_value_get_datetime_timezone(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), tz);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_datetime_timezone(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::DateTime(datetime) => {
                match CString::new(datetime.timezone_short_name().as_bytes()) {
                    Ok(str) => return str.into_raw(),
                    Err(err) => update_last_error(err),
                }
            }
            _ => new_error("Not a DateTime Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}
