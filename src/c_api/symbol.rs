// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Symbol](crate::val::Symbol) type.
//!

use super::err::{new_error, update_last_error};
use std::ffi::CString;
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Get number of character, without trailing zero, of a [Symbol](crate::val::Symbol) [Value](crate::val::Value)
/// # Arguments
/// val A [Symbol](crate::val::Symbol) [Value](crate::val::Value) to get the len from
/// # Returns
/// - the size of the string
/// - -1 (usize::MAX) if there was an error getting the string length
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::symbol::*;
/// # unsafe {
/// let symbol = std::ffi::CString::new("symbol").unwrap();
/// let val = haystack_value_make_symbol(symbol.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_symbol(val));
/// assert_eq!(haystack_value_get_symbol_value_len(val), 6);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_symbol_value_len(val: *const Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::Symbol(symbol) => return symbol.value.len(),
            _ => new_error("Not a Symbol Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Get the character of a [Symbol](crate::val::Symbol) [Value](crate::val::Value)
/// # Arguments
/// val A [Symbol](crate::val::Symbol) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::symbol::*;
/// # unsafe {
/// let symbol = std::ffi::CString::new("symbol").unwrap();
/// let val = haystack_value_make_symbol(symbol.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// let res = haystack_value_get_symbol_value(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), symbol);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_symbol_value(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::Symbol(symbol) => match CString::new(symbol.value.as_bytes()) {
                Ok(str) => return str.into_raw(),
                Err(err) => update_last_error(err),
            },
            _ => new_error("Not a Symbol Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}
