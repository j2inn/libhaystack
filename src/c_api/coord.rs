// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Coord](crate::val::Coord) type.
//!

use super::err::new_error;
use crate::haystack::val::Value;

/// Get the latitude of a  [Coord](crate::val::Coord) [Value](crate::val::Value)
/// # Arguments
/// val A [Coord](crate::val::Coord) [Value](crate::val::Value)
/// # Returns
/// - value
/// - NAN if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::coord::*;
/// # unsafe {
/// let val = haystack_value_make_coord(1.0, 2.0);
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_coord(val));
/// assert_eq!(haystack_value_get_coord_lat(val), 1.0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_coord_lat(val: *const Value) -> f64 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Coord(coord) => return coord.lat,
            _ => new_error("Not a Coord Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    f64::NAN
}

/// Get the longitude of a  [Coord](crate::val::Coord) [Value](crate::val::Value)
/// # Arguments
/// val A [Coord](crate::val::Coord) [Value](crate::val::Value)
/// # Returns
/// - value
/// - NAN if there was an error
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # use crate::libhaystack::c_api::coord::*;
/// # unsafe {
/// let val = haystack_value_make_coord(1.0, 2.0);
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_coord(val));
/// assert_eq!(haystack_value_get_coord_long(val), 2.0);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_get_coord_long(val: *const Value) -> f64 {
    match val.as_ref() {
        Some(value) => match value {
            Value::Coord(coord) => return coord.long,
            _ => new_error("Not a Coord Value"),
        },
        None => new_error("Invalid Value reference"),
    };
    f64::NAN
}
