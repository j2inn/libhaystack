// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Str](crate::val::Str) type.
//!

use super::err::{new_error, update_last_error};
use std::ffi::CString;
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Get number of character, without trailing zero, of a [Str](crate::val::Str) [Value](crate::val::Value)
/// # Arguments
/// val A [Str](crate::val::Str) [Value](crate::val::Value) to get the len from
/// # Returns
/// - the size of the string
/// - -1 (usize::MAX) if there was an error getting the string length, in which case use [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::str::*;
/// # unsafe {
/// let str = std::ffi::CString::new("foo bar").unwrap();
/// let val = haystack_value_make_str(str.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_str(val));
/// assert_eq!(haystack_value_get_str_len(val), 7);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_str_len(val: *const Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::Str(str) => return str.value.len(),
            _ => new_error("Not a Str Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Get the character of a [Str](crate::val::Str) [Value](crate::val::Value)
/// # Arguments
/// val A [Str](crate::val::Str) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::str::*;
/// # unsafe {
/// let str = std::ffi::CString::new("foo bar").unwrap();
/// let val = haystack_value_make_str(str.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_str(val));
/// let res = haystack_value_get_str_value(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), str);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_str_value(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::Str(str) => match CString::new(str.value.as_bytes()) {
                Ok(str) => return str.into_raw(),
                Err(err) => update_last_error(err),
            },
            _ => new_error("Not a Str Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}

/// Destructs and free a C Strings allocated by haystack functions
/// # Arguments
/// val C String pointer.
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_string_destroy(val: *mut c_char) {
    // Here the ownership of the pointer is taken by CString
    // which gets destructed at end of scope, so it will free the memory from the pointer also.
    let _ = CString::from_raw(val);
}
