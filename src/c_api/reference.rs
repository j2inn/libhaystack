// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Ref](crate::val::Ref) type.
//!

use super::err::{new_error, update_last_error};
use std::ffi::CString;
use std::os::raw::c_char;

use crate::haystack::val::Value;

/// Get number of character, without trailing zero, of a [Ref](crate::val::Ref) [Value](crate::val::Value)
/// # Arguments
/// val A [Ref](crate::val::Ref) [Value](crate::val::Value) to get the len from
/// # Returns
/// - the size of the string
/// - -1 (usize::MAX) if there was an error getting the string length
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::reference::*;
/// # unsafe {
/// let id = std::ffi::CString::new("someId").unwrap();
/// let val = haystack_value_make_ref(id.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(haystack_value_is_ref(val));
/// assert_eq!(haystack_value_get_ref_value_len(val), 6);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_ref_value_len(val: *const Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::Ref(uri) => return uri.value.len(),
            _ => new_error("Not a Uri Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Get the character of a [Ref](crate::val::Ref) [Value](crate::val::Value)
/// # Arguments
/// val A [Ref](crate::val::Ref) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::reference::*;
/// # unsafe {
/// let id = std::ffi::CString::new("someId").unwrap();
/// let val = haystack_value_make_ref(id.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// let res = haystack_value_get_ref_value(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), id);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_ref_value(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::Ref(uri) => match CString::new(uri.value.as_bytes()) {
                Ok(str) => return str.into_raw(),
                Err(err) => update_last_error(err),
            },
            _ => new_error("Not a Uri Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}

/// Get the character of a [Ref](crate::val::Ref) [Value](crate::val::Value)
/// # Arguments
/// val A [Ref](crate::val::Ref) [Value](crate::val::Value) to get the C string from.
/// # Returns
/// The C string or `null` if there was an error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::reference::*;
/// # unsafe {
/// let id = std::ffi::CString::new("someId").unwrap();
/// let dis = std::ffi::CString::new("Some Id").unwrap();
/// let val = haystack_value_make_ref_with_dis(id.as_ptr(), dis.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// let res = haystack_value_get_ref_value(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), id);
/// let res = haystack_value_get_ref_dis(val) as *mut i8;
/// assert_eq!(std::ffi::CString::from_raw(res), dis);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_ref_dis(val: *const Value) -> *const c_char {
    match val.as_ref() {
        Some(value) => match value {
            Value::Ref(uri) => match &uri.dis {
                Some(dis) => match CString::new(dis.as_bytes()) {
                    Ok(str) => return str.into_raw(),
                    Err(err) => update_last_error(err),
                },
                None => return std::ptr::null(),
            },
            _ => new_error("Not a Uri Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    std::ptr::null()
}
