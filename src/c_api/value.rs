// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! C API for working with the [Value](crate::val::Value) type.
//!

use super::err::new_error;
use crate::haystack::val::{Date, Dict, Grid, List, Time, Value};
use crate::safe_bool_call;
use crate::units::get_unit;
use chrono::{NaiveDateTime, TimeZone, Utc};

use std::ffi::CStr;
use std::os::raw::c_char;

/// Construct an empty [Value](crate::val::Value)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_init();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_null(val))
/// # }
/// ```
#[no_mangle]
pub extern "C" fn haystack_value_init() -> Box<Value> {
    Box::new(Value::default())
}

/// Destructs and free a [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
///
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_init();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// haystack_value_destroy(val);
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_destroy(val: *mut Value) {
    Box::from_raw(val);
}

/// True if a null (empty) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_init();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_null(val))
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_null(val: *const Value) -> bool {
    safe_bool_call!(val, is_null)
}

/// Construct a [Marker](crate::val::Marker) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_marker() -> Box<Value> {
    Box::new(Value::make_marker())
}

/// True if a [Marker](crate::val::Marker) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_marker();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(haystack_value_is_marker(val))
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_marker(val: *const Value) -> bool {
    safe_bool_call!(val, is_marker)
}

/// Construct a [NA](crate::val::Na) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_na() -> Box<Value> {
    Box::new(Value::make_na())
}

/// True if a [NA](crate::val::Na) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_na();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_na(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_na(val: *const Value) -> bool {
    safe_bool_call!(val, is_na)
}

/// Construct a [Remove](crate::val::Remove) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_remove() -> Box<Value> {
    Box::new(Value::make_remove())
}

/// True if a [Remove](crate::val::Remove) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_remove();
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_remove(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_remove(val: *const Value) -> bool {
    safe_bool_call!(val, is_remove)
}

/// Construct a [Bool](crate::val::Bool) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_bool(val: bool) -> Box<Value> {
    Box::new(Value::make_bool(val))
}

/// True if a [Bool](crate::val::Bool) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_bool(true);
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_na(val));
/// assert!(haystack_value_is_bool(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_bool(val: *const Value) -> bool {
    safe_bool_call!(val, is_bool)
}

/// Construct a [Number](crate::val::Number) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_number(val: f64) -> Box<Value> {
    Box::new(Value::make_number(val))
}

/// True if a [Number](crate::val::Number) [Value](crate::val::Value)
/// # Arguments
/// val a [Value](crate::val::Value).
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_number(12.0);
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_remove(val));
/// assert!(haystack_value_is_number(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_number(val: *const Value) -> bool {
    safe_bool_call!(val, is_number)
}

/// Construct a [Number](crate::val::Number) [Value](crate::val::Value) with an unit
/// # Arguments
/// - val a 64 bit double.
/// - a C string for the unit
/// # Returns
/// A Number [Value](crate::val::Value), if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let unit = std::ffi::CString::new("sec").unwrap();
/// let val = haystack_value_make_number_with_unit(42.0, unit.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_number(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_number_with_unit(
    val: f64,
    unit: *const c_char,
) -> Option<Box<Value>> {
    if unit.is_null() {
        new_error("Invalid null unit");
        return None;
    }
    match CStr::from_ptr(unit).to_str() {
        Ok(unit) => {
            if let Some(unit) = get_unit(unit) {
                Some(Box::new(Value::make_number_unit(val, unit)))
            } else {
                new_error(&format!("Invalid unit string. {unit}"));
                None
            }
        }
        Err(err) => {
            new_error(&format!("Invalid unit string. {err}"));
            None
        }
    }
}

/// Construct a [Coord](crate::val::Coord) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_coord(lat: f64, long: f64) -> Box<Value> {
    Box::new(Value::make_coord_from(lat, long))
}

/// True if a [Coord](crate::val::Coord) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// let val = haystack_value_make_coord(1.0, 2.0);
/// # unsafe {
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_coord(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_coord(val: *const Value) -> bool {
    safe_bool_call!(val, is_coord)
}

/// Construct a [Str](crate::val::Str) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_str(val: *const c_char) -> Option<Box<Value>> {
    if val.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(c_str) => Some(Box::new(Value::make_str(c_str))),
        Err(err) => {
            new_error(&format!("Invalid C string. {err}"));
            None
        }
    }
}

/// True if a [Str](crate::val::Str) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let str = std::ffi::CString::new("foo bar").unwrap();
/// let val = haystack_value_make_str(str.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_str(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_str(val: *const Value) -> bool {
    safe_bool_call!(val, is_str)
}

/// Construct a [Ref](crate::val::Ref) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_ref(val: *const c_char) -> Option<Box<Value>> {
    if val.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(c_str) => Some(Box::new(Value::make_ref(c_str))),
        Err(err) => {
            new_error(&format!("Invalid C string. {err}"));
            None
        }
    }
}

/// Construct a [Ref](crate::val::Ref) [Value](crate::val::Value) with a display name
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_ref_with_dis(
    val: *const c_char,
    dis: *const c_char,
) -> Option<Box<Value>> {
    if val.is_null() || dis.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(id) => match CStr::from_ptr(dis).to_str() {
            Ok(dis) => Some(Box::new(Value::make_ref_with_dis(id, dis))),
            Err(err) => {
                new_error(&format!("Invalid dis string. {err}"));
                None
            }
        },
        Err(err) => {
            new_error(&format!("Invalid value string. {err}"));
            None
        }
    }
}

/// True if a [Ref](crate::val::Ref) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let id = std::ffi::CString::new("someId_ofSomething").unwrap();
/// let val = haystack_value_make_ref(id.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_ref(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_ref(val: *const Value) -> bool {
    safe_bool_call!(val, is_ref)
}

/// Construct a [Uri](crate::val::Uri) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_uri(val: *const c_char) -> Option<Box<Value>> {
    if val.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(c_str) => Some(Box::new(Value::make_uri(c_str))),
        Err(err) => {
            new_error(&format!("Invalid C string. {err}"));
            None
        }
    }
}

/// True if a [Uri](crate::val::Uri) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let str = std::ffi::CString::new("/a/b/c").unwrap();
/// let val = haystack_value_make_uri(str.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_uri(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_uri(val: *const Value) -> bool {
    safe_bool_call!(val, is_uri)
}

/// Construct a [Symbol](crate::val::Symbol) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_symbol(val: *const c_char) -> Option<Box<Value>> {
    if val.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    match CStr::from_ptr(val).to_str() {
        Ok(c_str) => Some(Box::new(Value::make_symbol(c_str))),
        Err(err) => {
            new_error(&format!("Invalid C string. {err}"));
            None
        }
    }
}

/// True if a [Symbol](crate::val::Symbol) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let str = std::ffi::CString::new("def").unwrap();
/// let val = haystack_value_make_symbol(str.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_symbol(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_symbol(val: *const Value) -> bool {
    safe_bool_call!(val, is_symbol)
}

/// Construct a [XStr](crate::val::XStr) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_xstr(
    name: *const c_char,
    data: *const c_char,
) -> Option<Box<Value>> {
    if name.is_null() || data.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    let args: Result<Vec<&str>, std::str::Utf8Error> = [name, data]
        .iter()
        .map(|e| CStr::from_ptr(*e).to_str())
        .collect();

    match args {
        Ok(args) => Some(Box::new(Value::make_xstr_from(args[0], args[1]))),
        Err(err) => {
            new_error(&format!("Invalid C string arguments. {err}"));
            None
        }
    }
}

/// True if a [XStr](crate::val::XStr) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let typ = std::ffi::CString::new("Foo").unwrap();
/// let data = std::ffi::CString::new("bar").unwrap();
/// let val = haystack_value_make_xstr(typ.as_ptr(), data.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_xstr(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_xstr(val: *const Value) -> bool {
    safe_bool_call!(val, is_xstr)
}

/// Construct a [Time](crate::val::Time) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_time(hour: u32, min: u32, sec: u32) -> Option<Box<Value>> {
    match Time::from_hms(hour, min, sec) {
        Ok(time) => Some(Box::new(Value::Time(time))),
        Err(err) => {
            new_error(&err);
            None
        }
    }
}

/// Construct a [Time](crate::val::Time) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_time_millis(
    hour: u32,
    min: u32,
    sec: u32,
    milli: u32,
) -> Option<Box<Value>> {
    match Time::from_hms_milli(hour, min, sec, milli) {
        Ok(time) => Some(Box::new(Value::Time(time))),
        Err(err) => {
            new_error(&err);
            None
        }
    }
}

/// True if a [Time](crate::val::Time) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let val = haystack_value_make_time(12, 12, 59);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_time(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_time(val: *const Value) -> bool {
    safe_bool_call!(val, is_time)
}

/// Construct a [Date](crate::val::Date) [Value](crate::val::Value)
#[no_mangle]
pub extern "C" fn haystack_value_make_date(year: i32, month: u32, day: u32) -> Option<Box<Value>> {
    match Date::from_ymd(year, month, day) {
        Ok(date) => Some(Box::new(Value::Date(date))),
        Err(err) => {
            new_error(&err);
            None
        }
    }
}

/// True if a [Date](crate::val::Date) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let val = haystack_value_make_date(2021, 8, 13);
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_date(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_date(val: *const Value) -> bool {
    safe_bool_call!(val, is_date)
}

/// Construct a UTC [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_utc_datetime(
    date: *mut Value,
    time: *mut Value,
) -> Option<Box<Value>> {
    if date.is_null() || time.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }
    let args: Vec<&Value> = [date, time]
        .iter()
        .filter_map(|e| e.as_ref())
        .enumerate()
        .filter(|e| e.0 == 0 && e.1.is_date() || e.0 == 1 && e.1.is_time())
        .map(|e| e.1)
        .collect();

    if args.len() != 2 {
        new_error("Invalid null argument(s)");
        return None;
    }

    let (date, time) = (
        Date::try_from(args[0]).expect("Date"),
        Time::try_from(args[1]).expect("Time"),
    );

    let datetime = NaiveDateTime::new(*date, *time);
    let utc = Utc.from_utc_datetime(&datetime);

    Some(Box::new(Value::make_datetime(utc.into())))
}

/// Construct a Timezone [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
#[cfg(feature = "timezone")]
pub unsafe extern "C" fn haystack_value_make_tz_datetime(
    date: *mut Value,
    time: *mut Value,
    tz: *const c_char,
) -> Option<Box<Value>> {
    use crate::haystack::timezone::make_date_time_with_tz;
    use crate::haystack::val::DateTime;
    use chrono::Offset;

    let datetime = haystack_value_make_utc_datetime(date, time);

    if datetime.is_none() || tz.is_null() {
        new_error("Invalid null argument(s)");
        return None;
    }

    if let Ok(datetime) = if let Some(datetime) = datetime {
        DateTime::try_from(datetime.as_ref())
    } else {
        Err("")
    } {
        match CStr::from_ptr(tz).to_str() {
            Ok(tz) => match make_date_time_with_tz(&datetime.with_timezone(&Utc.fix()), tz) {
                Ok(datetime) => Some(Box::new(Value::DateTime(datetime.into()))),
                Err(err) => {
                    new_error(&err);
                    None
                }
            },
            Err(err) => {
                new_error(&format!("Invalid timezone string. {err}"));
                None
            }
        }
    } else {
        new_error("Invalid datetime");
        None
    }
}

/// True if a [DateTime](crate::val::DateTime) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let date = haystack_value_make_date(2021, 8, 13);
/// let time = haystack_value_make_time(9, 45, 59);
/// # let date = Box::<Value>::leak(date.unwrap());
/// # let time = Box::<Value>::leak(time.unwrap());
/// let tz = std::ffi::CString::new("UTC").unwrap();
/// let val = haystack_value_make_tz_datetime(date, time, tz.as_ptr());
/// # let val = Box::<Value>::leak(val.unwrap());
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_datetime(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_datetime(val: *const Value) -> bool {
    safe_bool_call!(val, is_datetime)
}

/// Construct a empty [List](crate::val::List) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_list() -> Box<Value> {
    Box::new(Value::make_list(List::new()))
}

/// True if a [List](crate::val::List) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let val = haystack_value_make_list();
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_list(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_list(val: *const Value) -> bool {
    safe_bool_call!(val, is_list)
}

/// Construct an empty [Dict](crate::val::Dict) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_dict() -> Box<Value> {
    Box::new(Value::make_dict(Dict::new()))
}

/// True if a [Dict](crate::val::Dict) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let val = haystack_value_make_dict();
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_dict(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_dict(val: *const Value) -> bool {
    safe_bool_call!(val, is_dict)
}

/// Construct an empty [Grid](crate::val::Grid) [Value](crate::val::Value)
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_make_grid() -> Box<Value> {
    Box::new(Value::make_grid(Grid::make_empty()))
}

/// True if a [Grid](crate::val::Grid) [Value](crate::val::Value)
/// # Returns
/// True, or False in which case if argument type mismatch an error is created that can be read using
/// [last_error_message](super::err::last_error_message)
/// # Example
/// ```rust
/// # use crate::libhaystack::val::Value;
/// # use crate::libhaystack::c_api::value::*;
/// # unsafe {
/// let val = haystack_value_make_grid();
/// # let val = Box::<Value>::leak(val);
/// assert!(!haystack_value_is_null(val));
/// assert!(haystack_value_is_grid(val));
/// # }
/// ```
/// # Safety
/// Panics on invalid input data
#[no_mangle]
pub unsafe extern "C" fn haystack_value_is_grid(val: *const Value) -> bool {
    safe_bool_call!(val, is_grid)
}
