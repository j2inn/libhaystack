// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Grid](crate::val::Grid) type.
//!

use super::{err::new_error, ResultType};
use crate::{haystack::val::Value, val::Dict};

/// Get number of rows of a [Grid](crate::val::Grid) [Value](crate::val::Value)
/// # Arguments
/// val A [Grid](crate::val::Grid) [Value](crate::val::Value)
/// # Returns
/// - number of elements
/// - -1 if there was an error getting the length
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # unsafe {
/// let val = haystack_value_make_grid();
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_grid(val));
/// assert_eq!(haystack_value_get_grid_len(val), 0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_grid_len(val: *const Value) -> usize {
    match val.as_ref() {
        Some(value) => match value {
            Value::Grid(grid) => return grid.len(),
            _ => new_error("Not a Grid Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    usize::MAX
}

/// Construct a [Grid](crate::val::Grid) [Value](crate::val::Value) from a [List](crate::val::List) [Value](crate::val::Value) of
/// [Dict](crate::val::Dict) [Value](crate::val::Value)s
/// # Arguments
/// - rows List](crate::val::List) [Value](crate::val::Value) of
///   [Dict](crate::val::Dict) [Value](crate::val::Value)s
/// # Returns
/// - The value pointer
/// - None if there was an error, in which case use [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// // Make a row
/// let dict = haystack_value_make_dict();
/// # let dict = Box::<Value>::leak(dict);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(dict, key.as_ptr(), entry);
/// // Make a list of rows
/// let list = haystack_value_make_list();
/// # let list = Box::<Value>::leak(list);
/// haystack_value_push_list_entry(list, dict);
/// // Make the Grid
/// let grid = haystack_value_make_grid_from_rows(list);
/// # let grid = Box::<Value>::leak(grid.unwrap());
/// assert!(grid.is_grid());
/// assert_eq!(haystack_value_get_grid_len(grid), 1);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_grid_from_rows(
    rows: *const Value,
) -> Option<Box<Value>> {
    match rows.as_ref() {
        Some(rows) => match rows {
            Value::List(list) => {
                let rows = list
                    .iter()
                    .filter_map(|v| match v {
                        Value::Dict(dict) => Some(dict.clone()),
                        _ => None,
                    })
                    .collect::<Vec<Dict>>();
                if !rows.is_empty() {
                    Some(Box::from(Value::make_grid_from_dicts(rows)))
                } else {
                    new_error("Specified rows list must be of Dict type Values");
                    None
                }
            }
            _ => {
                new_error("Not a List Value");
                None
            }
        },
        None => {
            new_error("Invalid Value reference");
            None
        }
    }
}

/// Construct a [Grid](crate::val::Grid) [Value](crate::val::Value) from a [List](crate::val::List) [Value](crate::val::Value) of
/// [Dict](crate::val::Dict) [Value](crate::val::Value)s
/// # Arguments
/// - rows a [List](crate::val::List) [Value](crate::val::Value) of
///   [Dict](crate::val::Dict) [Value](crate::val::Value)s
/// # Returns
/// - The value pointer
/// - None if there was an error, in which case use [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// // Make a row
/// let dict = haystack_value_make_dict();
/// # let dict = Box::<Value>::leak(dict);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(dict, key.as_ptr(), entry);
/// // Make a list of rows
/// let list = haystack_value_make_list();
/// # let list = Box::<Value>::leak(list);
/// haystack_value_push_list_entry(list, dict);
/// // Make the Grid
/// let grid = haystack_value_make_grid_from_rows(list);
/// # let grid = Box::<Value>::leak(grid.unwrap());
/// assert!(grid.is_grid());
/// assert_eq!(haystack_value_get_grid_len(grid), 1);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_grid_from_rows_with_meta(
    rows: *const Value,
    meta: *const Value,
) -> Option<Box<Value>> {
    match haystack_value_make_grid_from_rows(rows) {
        Some(mut val) => match val.as_mut() {
            Value::Grid(grid) => {
                if let Some(Value::Dict(meta)) = meta.as_ref() {
                    grid.meta = Some(meta.clone());
                    Some(Box::from(Value::from(grid.clone())))
                } else {
                    new_error("Not a Grid Value");
                    None
                }
            }
            _ => {
                new_error("Not a Grid Value");
                None
            }
        },
        None => {
            new_error("Not a Grid Value");
            None
        }
    }
}

/// Get the row of a [Grid](crate::val::Grid) [Value](crate::val::Value) at the specified index
/// # Arguments
/// val A [Grid](crate::val::Grid) [Value](crate::val::Value)
/// # Returns
/// - The row at the specified index, or None if there was an error, in which case use [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # unsafe {
/// // Make a row
/// let dict = haystack_value_make_dict();
/// # let dict = Box::<Value>::leak(dict);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(dict, key.as_ptr(), entry);
/// // Make a list of rows
/// let list = haystack_value_make_list();
/// # let list = Box::<Value>::leak(list);
/// haystack_value_push_list_entry(list, dict);
/// // Make the Grid
/// let grid = haystack_value_make_grid_from_rows(list);
/// # let grid = Box::<Value>::leak(grid.unwrap());
/// let result = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_value_get_grid_row_at(grid, 0, result), ResultType::TRUE);
/// assert_eq!(*result, *dict);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_grid_row_at(
    val: *mut Value,
    index: usize,
    result: *mut Value,
) -> ResultType {
    match val.as_ref() {
        Some(value) => match value {
            Value::Grid(grid) => match grid.rows.get(index) {
                Some(entry) => {
                    if let Some(value) = result.as_mut() {
                        *value = Value::Dict(entry.clone());
                        return ResultType::TRUE;
                    } else {
                        new_error("Not a Value result.");
                    }
                }
                None => new_error("List Index out of bounds"),
            },
            _ => new_error("Not a Grid Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    ResultType::ERR
}
