// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working JSON encoding.
//!

use super::err::{new_error, update_last_error};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Encodes a [Value](crate::val::Value) to a JSON string
/// # Arguments
/// - val Value to encode as a JSON string
/// # Return
/// a `C string` with the JSON encoded value
/// or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_to_json_string(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(val) => match serde_json::to_string(val) {
            Ok(str) => match CString::new(str.as_str()) {
                Ok(str) => str.into_raw(),
                Err(err) => {
                    update_last_error(err);
                    std::ptr::null()
                }
            },
            Err(err) => {
                update_last_error(err);
                std::ptr::null()
            }
        },
        None => {
            new_error("Invalid value");
            std::ptr::null()
        }
    }
}

/// Decodes a [Value](crate::val::Value) from a JSON string
/// # Arguments
/// - val `C string` with the JSON encoded value
/// # Return
/// the decoded [Value](crate::val::Value) or `null` if there was an error,
/// in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_from_json_string(
    input: *const c_char,
) -> Option<Box<Value>> {
    if input.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(input).to_str() {
        Ok(c_str) => match serde_json::from_str::<Value>(c_str) {
            Ok(val) => Some(Box::new(val)),
            Err(err) => {
                update_last_error(err);
                None
            }
        },
        Err(err) => {
            new_error(&format!("Invalid C string. {err}"));
            None
        }
    }
}
