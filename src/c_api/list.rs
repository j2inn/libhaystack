// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [List](crate::val::List) type.
//!

use super::{err::new_error, ResultType};

use crate::haystack::val::Value;

/// Get number of elements of a [List](crate::val::List) [Value](crate::val::Value)
/// # Arguments
/// val A [List](crate::val::List) [Value](crate::val::Value)
/// # Returns
/// - number of elements
/// - -1 if there was an error getting the length
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_list(val));
/// assert_eq!(haystack_value_get_list_len(val), 0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_list_len(val: *mut Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::List(list) => return list.len(),
            _ => new_error("Not a List Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Push an element to a [List](crate::val::List) [Value](crate::val::Value)
/// # Arguments
/// - val A [List](crate::val::List) [Value](crate::val::Value)
/// - entry A [Value](crate::val::Value) to be inserted into the list
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_list(val));
/// # let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// assert_eq!(haystack_value_push_list_entry(val, entry), ResultType::TRUE);
/// assert_eq!(haystack_value_get_list_len(val), 1);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_push_list_entry(
    val: *mut Value,
    entry: *const Value,
) -> ResultType {
    match val.as_mut() {
        Some(value) => match value {
            Value::List(list) => match entry.as_ref() {
                Some(entry) => {
                    list.push(entry.clone());
                    return ResultType::TRUE;
                }
                None => new_error("Invalid Entry reference"),
            },
            _ => new_error("Not a List Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Get the element of a [List](crate::val::List) [Value](crate::val::Value) at the specified index
/// # Arguments
/// - val A [List](crate::val::List) [Value](crate::val::Value)
/// - result The Value to be updated with the result.
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_list(val));
/// # let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// haystack_value_push_list_entry(val, entry);
/// let mut result = std::ptr::null();
/// assert_eq!(haystack_value_get_list_entry_at(val, 0, &mut result), ResultType::TRUE);
/// assert_eq!(*result, *entry);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_list_entry_at(
    val: *const Value,
    index: usize,
    result: *mut *const Value,
) -> ResultType {
    match val.as_ref() {
        Some(value) => match value {
            Value::List(list) => match list.get(index) {
                Some(entry) => {
                    if let Some(value) = result.as_mut() {
                        *value = entry;
                        return ResultType::TRUE;
                    } else {
                        new_error("Not a Value result.");
                    }
                }
                None => new_error("List Index out of bounds"),
            },
            _ => new_error("Not a List Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Set the element of a [List](crate::val::List) [Value](crate::val::Value) at the specified index
/// # Arguments
/// - val A [List](crate::val::List) [Value](crate::val::Value)
/// - index The index were to inset the value at
/// - element  A  [Value](crate::val::Value)
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_list(val));
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// haystack_value_push_list_entry(val, entry);
/// let mut result = std::ptr::null();
/// assert_eq!(haystack_value_get_list_entry_at(val, 0, &mut result), ResultType::TRUE);
/// assert_eq!(*result, *entry);
/// let entry = haystack_value_make_remove();
/// # let entry = Box::<Value>::leak(entry);
/// haystack_value_set_list_entry_at(val, 0, entry);
/// assert_eq!(haystack_value_get_list_entry_at(val, 0, &mut result), ResultType::TRUE);
/// assert_eq!(*result, *entry);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_set_list_entry_at(
    val: *mut Value,
    index: usize,
    entry: *mut Value,
) -> ResultType {
    match val.as_mut() {
        Some(value) => match value {
            Value::List(list) => {
                if index < list.len() {
                    match entry.as_ref() {
                        Some(entry) => {
                            list.insert(index, entry.clone());
                            return ResultType::TRUE;
                        }
                        None => new_error("Invalid Entry reference"),
                    }
                } else {
                    new_error("List Index out of bounds")
                }
            }
            _ => new_error("Not a List Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}

/// Remove the element of a [List](crate::val::List) [Value](crate::val::Value) at the specified index
/// # Arguments
/// - val A [List](crate::val::List) [Value](crate::val::Value)
/// - index The index were to inset the value at
/// # Returns
/// - 1 (True) if the operation was successful, -1 otherwise in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_list(val));
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// haystack_value_push_list_entry(val, entry);
/// let mut result = std::ptr::null();
/// assert_eq!(haystack_value_get_list_entry_at(val, 0, &mut result), ResultType::TRUE);
/// haystack_value_remove_list_entry_at(val, 0);
/// assert_eq!(haystack_value_get_list_len(val), 0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_remove_list_entry_at(
    val: *mut Value,
    index: usize,
) -> ResultType {
    match val.as_mut() {
        Some(value) => match value {
            Value::List(list) => {
                if index < list.len() {
                    list.remove(index);
                    return ResultType::TRUE;
                } else {
                    new_error("List Index out of bounds")
                }
            }
            _ => new_error("Not a List Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}
