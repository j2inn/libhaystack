// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Dict](crate::val::Dict) type.
//!

use std::{ffi::CStr, os::raw::c_char};

use super::{
    err::{new_error, update_last_error},
    ResultType,
};

use crate::{haystack::val::Value, val::List};

/// Get number of elements of a [Dict](crate::val::Dict) [Value](crate::val::Value)
/// # Arguments
/// val A [Dict](crate::val::Dict) [Value](crate::val::Value)
/// # Returns
/// - number of elements
/// - -1 if there was an error getting the length, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// let val = haystack_value_make_dict();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_dict(val));
/// assert_eq!(haystack_value_get_dict_len(val), 0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_dict_len(val: *const Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::Dict(dict) => return dict.len(),
            _ => new_error("Not a Dict Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Get the keys of a [Dict](crate::val::Dict) [Value](crate::val::Value)
/// as a [List](crate::val::List) [Value](crate::val::Value)
/// # Arguments
/// - val A [Dict](crate::val::Dict) [Value](crate::val::Value)
/// - result The Value to be updated with the result Str List.
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let dict = haystack_value_make_dict();
/// # let dict = Box::<Value>::leak(dict);
/// assert!(haystack_value_is_dict(dict));
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(dict, key.as_ptr(), entry);
/// let keys = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_value_get_dict_keys(dict, keys), ResultType::TRUE);
/// assert_eq!(haystack_value_get_list_len(keys), 1);
/// let mut  result = std::ptr::null();
/// assert_eq!(haystack_value_get_list_entry_at(keys, 0, &mut result), ResultType::TRUE);
/// # assert_eq!((*result).to_string(), String::from("\"equip\""));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_dict_keys(
    val: *const Value,
    result: *mut Value,
) -> ResultType {
    match val.as_ref() {
        Some(value) => match value {
            Value::Dict(dict) => {
                let keys: Value = dict
                    .keys()
                    .map(|v| Value::make_str(v))
                    .collect::<List>()
                    .into();
                if let Some(value) = result.as_mut() {
                    *value = keys;
                    return ResultType::TRUE;
                } else {
                    new_error("Not a Value result.");
                }
            }
            _ => new_error("Not a Dict Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Insert an new element or updates existing key of a [Dict](crate::val::Dict) [Value](crate::val::Value)
/// # Arguments
/// - val A [Dict](crate::val::Dict) [Value](crate::val::Value)
/// - key A C string for the dict key, must be valid Haystack key.
/// - entry A [Value](crate::val::Value) to be inserted into the dict
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// let val = haystack_value_make_dict();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_dict(val));
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// assert_eq!(haystack_value_insert_dict_entry(val, key.as_ptr(), entry), ResultType::TRUE);
/// assert_eq!(haystack_value_get_dict_len(val), 1);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_insert_dict_entry(
    val: *mut Value,
    key: *const c_char,
    entry: *const Value,
) -> ResultType {
    if val.is_null() || key.is_null() || entry.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }

    match CStr::from_ptr(key).to_str() {
        Ok(key) => match val.as_mut() {
            Some(value) => match value {
                Value::Dict(dict) => match entry.as_ref() {
                    Some(entry) => {
                        dict.insert(key.to_string(), entry.clone());
                        return ResultType::TRUE;
                    }
                    None => new_error("Invalid Entry reference"),
                },
                _ => new_error("Not a Dict Value"),
            },
            None => new_error("Invalid Value reference"),
        },
        Err(err) => update_last_error(err),
    }

    ResultType::ERR
}

/// Get the element of a [Dict](crate::val::Dict) [Value](crate::val::Value) by the key
/// # Arguments
/// - val A [Dict](crate::val::Dict) [Value](crate::val::Value)
/// - result The Value to be updated.
/// # Returns
/// - 1 (True) if the operation was successful, 0 (false) if no element found, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// let val = haystack_value_make_dict();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_dict(val));
/// # let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(val, key.as_ptr(), entry);
/// assert_eq!(haystack_value_get_dict_len(val), 1);
/// let mut result = std::ptr::null();
/// assert_eq!(haystack_value_get_dict_entry(val, key.as_ptr(), &mut result), ResultType::TRUE);
/// assert_eq!(&*result, entry);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_dict_entry(
    val: *mut Value,
    key: *const c_char,
    result: *mut *const Value,
) -> ResultType {
    if val.is_null() || key.is_null() || result.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }

    match CStr::from_ptr(key).to_str() {
        Ok(key) => match val.as_ref() {
            Some(Value::Dict(dict)) => match dict.get(key) {
                Some(val) => {
                    if let Some(value) = result.as_mut() {
                        *value = val;
                        return ResultType::TRUE;
                    } else {
                        new_error("Not a Value result.");
                    }
                }
                None => return ResultType::FALSE,
            },
            _ => new_error("Invalid Dict Value reference"),
        },
        Err(err) => update_last_error(err),
    };
    ResultType::ERR
}

/// Remove the element of a [Dict](crate::val::Dict) [Value](crate::val::Value) for the key, if it exists.
/// # Arguments
/// - val A [Dict](crate::val::Dict) [Value](crate::val::Value)
/// - key The key to be removed
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// let val = haystack_value_make_dict();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_dict(val));
/// # let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(val, key.as_ptr(), entry);
/// assert_eq!(haystack_value_get_dict_len(val), 1);
/// assert_eq!(haystack_value_remove_dict_entry(val, key.as_ptr()), ResultType::TRUE);
/// assert_eq!(haystack_value_get_dict_len(val), 0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_remove_dict_entry(
    val: *mut Value,
    key: *const c_char,
) -> ResultType {
    if val.is_null() || key.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }
    match CStr::from_ptr(key).to_str() {
        Ok(key) => match val.as_mut() {
            Some(Value::Dict(dict)) => {
                dict.remove(key);
                ResultType::TRUE
            }
            _ => {
                new_error("Invalid Dict Value reference");
                ResultType::ERR
            }
        },
        Err(err) => {
            update_last_error(err);
            ResultType::ERR
        }
    }
}
