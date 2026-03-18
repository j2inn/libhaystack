// Copyright (C) 2020 - 2024, J2 Innovations

//!
//! C API for Trio text encoding.
//!

use super::err::{new_error, update_last_error};
use crate::haystack::encoding::trio::decode::TrioReader;
use crate::haystack::encoding::trio::encode::TrioWriter;
use crate::haystack::val::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Encodes a [Dict](crate::val::Dict) or [Grid](crate::val::Grid)
/// [Value](crate::val::Value) as a Trio-encoded string.
///
/// For a `Dict` value the dict is serialised as a single Trio record.
/// For a `Grid` value every row is serialised as an individual Trio record.
///
/// # Return
/// A null-terminated C string owned by the caller, or `null` on error.
/// The caller must free the returned string with
/// [`haystack_string_free`](super::str::haystack_string_free).
///
/// On error, [`last_error_message`](super::err::last_error_message) returns
/// the reason.
///
/// # Safety
/// Panics on invalid input data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haystack_value_to_trio_string(val: *const Value) -> *const c_char {
    match unsafe { val.as_ref() } {
        Some(Value::Dict(dict)) => {
            let s = TrioWriter::dict_to_trio(dict, None);
            match CString::new(s.as_str()) {
                Ok(cs) => cs.into_raw(),
                Err(err) => {
                    update_last_error(err);
                    std::ptr::null()
                }
            }
        }
        Some(Value::Grid(grid)) => {
            let s = TrioWriter::new().add_grid(grid).to_trio_string();
            match CString::new(s.as_str()) {
                Ok(cs) => cs.into_raw(),
                Err(err) => {
                    update_last_error(err);
                    std::ptr::null()
                }
            }
        }
        Some(_) => {
            new_error("Trio encoding requires a Dict or Grid value");
            std::ptr::null()
        }
        None => {
            new_error("Invalid value");
            std::ptr::null()
        }
    }
}

/// Decodes the first [Dict](crate::val::Dict) record from a Trio-encoded
/// string and returns it as a [Value](crate::val::Value).
///
/// # Return
/// The decoded `Dict` wrapped in a [Value](crate::val::Value), or `null` on
/// error.  Call [`last_error_message`](super::err::last_error_message) to
/// retrieve the error description.
///
/// # Safety
/// Panics on invalid input data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haystack_value_from_trio_string(
    input: *const c_char,
) -> Option<Box<Value>> {
    if input.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }

    match unsafe { CStr::from_ptr(input).to_str() } {
        Ok(c_str) => match TrioReader::dicts_from_str(c_str) {
            Ok(mut dicts) if !dicts.is_empty() => Some(Box::new(Value::from(dicts.remove(0)))),
            Ok(_) => {
                new_error("No dicts found in Trio input");
                None
            }
            Err(err) => {
                update_last_error(err);
                None
            }
        },
        Err(err) => {
            update_last_error(err);
            None
        }
    }
}
