// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for [Filter](crate::filter::Filter) type.
//!

use crate::filter::{Filter, Filtered, ListFiltered};
use crate::val::{Grid, Value};
use std::{ffi::CStr, os::raw::c_char};

use super::err::{new_error, update_last_error};
use super::ResultType;

/// Parses a [Filter](crate::filter::Filter) from  a C string.
/// # Arguments
/// val The Haystack filter as a C string.
/// # Returns
/// The Filter object or None if error, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::filter::Filter;
/// # use crate::libhaystack::c_api::filter::*;
/// # unsafe {
/// let str = std::ffi::CString::new("site and dis==\"Some site\"").unwrap();
/// let filter = haystack_filter_parse(str.as_ptr());
/// # let filter = Box::<Filter>::into_raw(filter.unwrap());
/// assert_ne!(filter, std::ptr::null_mut());
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_filter_parse(val: *const c_char) -> Option<Box<Filter>> {
    if val.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(c_str) => match Filter::try_from(c_str) {
            Ok(filter) => Some(Box::new(filter)),
            Err(err) => {
                update_last_error(err);
                None
            }
        },
        Err(err) => {
            new_error(&format!("Invalid C string. {}", err));
            None
        }
    }
}

/// Uses a [Filter](crate::filter::Filter) to match against a [Dict](crate::val::Dict)
/// [Value](crate::val::Value).
/// # Arguments
/// - filter The Haystack filter.
/// - val The Dict Value.
/// # Returns
/// 1 (True) if filter matches, 0 (false) when no march, or -1 when error is encountered, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::filter::Filter;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::filter::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let str = std::ffi::CString::new("equip").unwrap();
/// let filter = haystack_filter_parse(str.as_ptr());
/// # let filter = Box::<Filter>::into_raw(filter.unwrap());
/// let dict = haystack_value_make_dict();
/// # let dict = Box::<Value>::leak(dict);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(dict, key.as_ptr(), entry);
/// assert!(haystack_filter_match_dict(filter, dict) == ResultType::TRUE);
///
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_filter_match_dict(
    filter: *const Filter,
    dict: *const Value,
) -> ResultType {
    if filter.is_null() || dict.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }

    match filter.as_ref() {
        Some(filter) => {
            if let Some(Value::Dict(dict)) = dict.as_ref() {
                return if dict.filter(filter) {
                    ResultType::TRUE
                } else {
                    ResultType::FALSE
                };
            } else {
                new_error("Not a Dict.");
            }
        }
        None => {
            new_error("Not a Filter.");
        }
    };
    ResultType::ERR
}

/// Uses a [Filter](crate::filter::Filter) to match against a [Grid](crate::val::Grid) [Value](crate::val::Value) and return
/// first matching record, or None if no matches found.
/// # Arguments
/// - filter The Haystack filter.
/// - val The Grid Value.
/// - result The Value to be updated with the result Dict.
/// # Returns
/// 1 (True) if filter matches, 0 (false) when no march, or -1 when error is encountered, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::list::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # use crate::libhaystack::filter::Filter;
/// # use crate::libhaystack::c_api::filter::*;
/// # use crate::libhaystack::c_api::zinc::*;
/// # unsafe {
/// // Make first row
/// let row1 = haystack_value_make_dict();
/// # let row1 = Box::<Value>::leak(row1);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("equip").unwrap();
/// haystack_value_insert_dict_entry(row1, key.as_ptr(), entry);
///
/// // Make second row
/// let row2 = haystack_value_make_dict();
/// # let row2 = Box::<Value>::leak(row2);
/// let entry = haystack_value_make_marker();
/// # let entry = Box::<Value>::leak(entry);
/// let key = std::ffi::CString::new("site").unwrap();
/// haystack_value_insert_dict_entry(row2, key.as_ptr(), entry);
///
/// // Make a list of rows
/// let list = haystack_value_make_list();
/// # let list = Box::<Value>::leak(list);
/// haystack_value_push_list_entry(list, row1);
/// haystack_value_push_list_entry(list, row2);
///
/// // Make the Grid
/// let grid = haystack_value_make_grid_from_rows(list);
/// # let grid = Box::<Value>::leak(grid.unwrap());
///
/// // Make a filter
/// let str = std::ffi::CString::new("equip").unwrap();
/// let filter = haystack_filter_parse(str.as_ptr());
/// # let filter = Box::<Filter>::into_raw(filter.unwrap());
/// # let result = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_filter_first_match_in_grid(filter, grid, result), ResultType::TRUE);
/// # assert_eq!((*result).to_string(), (*row1).to_string());
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_filter_first_match_in_grid(
    filter: *const Filter,
    grid: *const Value,
    result: *mut Value,
) -> ResultType {
    if filter.is_null() || result.is_null() || grid.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }

    match filter.as_ref() {
        Some(filter) => {
            if let Some(Value::Grid(grid)) = grid.as_ref() {
                return match grid.filter(filter) {
                    Some(dict) => {
                        if let Some(value) = result.as_mut() {
                            *value = Value::Dict(dict.to_owned());
                            ResultType::TRUE
                        } else {
                            new_error("Not a Value result.");
                            ResultType::ERR
                        }
                    }
                    None => ResultType::FALSE,
                };
            } else {
                new_error("Not a Grid.");
            }
        }
        None => {
            new_error("Not a Filter.");
        }
    };
    ResultType::ERR
}

/// Uses a [Filter](crate::filter::Filter) to match against a [Grid](crate::val::Grid) [Value](crate::val::Value) and return a
/// new [Grid](crate::val::Grid) [Value](crate::val::Value) with the matching records.
/// # Arguments
/// - filter The Haystack filter.
/// - val The Grid Value.
/// - result The Value to be updated with the result Grid.
/// # Returns
/// 1 (True) if filter matches, 0 (false) when no march, or -1 when error is encountered, in which case the [last_error_message](super::err::last_error_message)
/// can be called to get the error message.
/// # Example
/// ```rust
/// # use crate::libhaystack::val::{Value, Dict};
/// # use crate::libhaystack::dict;
/// # use crate::libhaystack::c_api::ResultType;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::list::*;
/// # use crate::libhaystack::c_api::grid::*;
/// # use crate::libhaystack::c_api::dict::*;
/// # use crate::libhaystack::filter::Filter;
/// # use crate::libhaystack::c_api::filter::*;
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
/// // Make a filter
/// let str = std::ffi::CString::new("equip").unwrap();
/// let filter = haystack_filter_parse(str.as_ptr());
/// # let filter = Box::<Filter>::into_raw(filter.unwrap());
/// let mut result = Box::into_raw(haystack_value_init());
/// assert_eq!(haystack_filter_match_all_grid(filter, grid, result), ResultType::TRUE);
/// # let test = Value::make_grid_from_dicts(vec![dict!{"equip" => Value::Marker}]);
/// # assert_eq!((*result).to_string(), test.to_string());
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_filter_match_all_grid(
    filter: *const Filter,
    grid: *const Value,
    result: *mut Value,
) -> ResultType {
    if filter.is_null() || grid.is_null() || result.is_null() {
        new_error("Invalid null argument(s)");
        return ResultType::ERR;
    }

    match filter.as_ref() {
        Some(filter) => {
            if let Some(Value::Grid(grid)) = grid.as_ref() {
                let rows = grid.filter_all(filter);
                let res = if let Some(meta) = &grid.meta {
                    Grid::make_from_dicts_with_meta(
                        rows.into_iter().cloned().collect(),
                        meta.clone(),
                    )
                } else {
                    Grid::make_from_dicts(rows.into_iter().cloned().collect())
                };
                if let Some(value) = result.as_mut() {
                    let ret = if !res.is_empty() {
                        ResultType::TRUE
                    } else {
                        ResultType::FALSE
                    };
                    *value = Value::Grid(res);
                    return ret;
                } else {
                    new_error("Not a Value result.");
                }
            } else {
                new_error("Not a Grid.");
            }
        }
        None => {
            new_error("Not a Filter.");
        }
    };
    ResultType::ERR
}
