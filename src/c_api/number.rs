// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Number](crate::val::Number) type.
//!

use super::err::{new_error, update_last_error};
use super::ResultType;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Get value of a [Number](crate::val::Number) [Value](crate::val::Value)
/// # Arguments
/// val A [Number](crate::val::Number) [Value](crate::val::Value)
/// # Returns
/// - value
/// - NAN if there was an error getting the string length
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::number::*;
/// # unsafe {
/// let val = haystack_value_make_number(100.0);
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_number(val));
/// assert_eq!(haystack_value_get_number_value(val), 100.0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_number_value(val: *const Value) -> f64 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Number(num) => return num.value,
            _ => new_error("Not a Number Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    f64::NAN
}

/// True if [Number](crate::val::Number) [Value](crate::val::Value) has an unit
/// # Arguments
/// val A [Number](crate::val::Number) [Value](crate::val::Value) to test
/// # Returns
/// - 1 if has unit, 0 otherwise
/// - -1 if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::number::*;
/// # unsafe {
/// let val = haystack_value_make_number(100.0);
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_number(val));
/// assert!(haystack_value_number_has_unit(val) == ResultType::FALSE);
/// let unit = std::ffi::CString::new("sec").unwrap();
/// let val = haystack_value_make_number_with_unit(42.0, unit.as_ptr()).unwrap();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_number(val));
/// assert!(haystack_value_number_has_unit(val) == ResultType::TRUE);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_number_has_unit(val: *const Value) -> ResultType {
    match val.as_ref() {
        Some(value) => match value {
            Value::Number(num) => {
                return if num.unit.is_some() {
                    ResultType::TRUE
                } else {
                    ResultType::FALSE
                }
            }
            _ => new_error("Not a Number Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Get the character of a [Number](crate::val::Number) [Value](crate::val::Value) unit
/// # Arguments
/// val A [Number](crate::val::Number) [Value](crate::val::Value) to get the unit from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::number::*;
/// let unit = std::ffi::CString::new("s").unwrap();
/// # unsafe {
/// let val = haystack_value_make_number_with_unit(42.0, unit.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_number(val));
/// let res = haystack_value_get_number_unit(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), unit);
/// }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_number_unit(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::Number(num) => match &num.unit {
                Some(unit) => match CString::new(unit.symbol().as_bytes()) {
                    Ok(str) => return str.into_raw(),
                    Err(err) => update_last_error(err),
                },
                None => return std::ptr::null(),
            },
            _ => new_error("Not a Str Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}
