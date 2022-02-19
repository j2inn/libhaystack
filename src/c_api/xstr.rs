// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [XStr](crate::val::XStr) type.
//!

use super::err::{new_error, update_last_error};
use std::ffi::CString;
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Get the characters of a [XStr](crate::val::XStr) [Value](crate::val::Value) type
/// # Arguments
/// val A [XStr](crate::val::XStr) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::xstr::*;
/// # unsafe {
/// let name = std::ffi::CString::new("Type").unwrap();
/// let data = std::ffi::CString::new("data").unwrap();
/// let val = haystack_value_make_xstr(name.as_ptr(), data.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// let res = haystack_value_get_xstr_type(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), name);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_xstr_type(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::XStr(xstr) => match CString::new(xstr.r#type.as_bytes()) {
                Ok(str) => return str.into_raw(),
                Err(err) => update_last_error(err),
            },
            _ => new_error("Not a XStr Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}

/// Get the characters of a [XStr](crate::val::XStr) [Value](crate::val::Value)
/// # Arguments
/// val A [XStr](crate::val::XStr) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::xstr::*;
/// # unsafe {
/// let name = std::ffi::CString::new("Type").unwrap();
/// let data = std::ffi::CString::new("data").unwrap();
/// let val = haystack_value_make_xstr(name.as_ptr(), data.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// let res = haystack_value_get_xstr_value(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), data);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_xstr_value(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::XStr(xstr) => match CString::new(xstr.value.as_bytes()) {
                Ok(str) => return str.into_raw(),
                Err(err) => update_last_error(err),
            },
            _ => new_error("Not a XStr Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}
