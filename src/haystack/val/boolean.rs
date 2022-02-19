// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Bool

use crate::haystack::val::Value;

/// Haystack `Bool`
///
/// It holds a boolean `true` or `false`
///
/// # Example
///  Create a Bool from boolean value
/// ```
/// let bool = libhaystack::val::Bool::from(true);
/// assert!(bool.value);
/// ```
///
/// Create a Haystack [Value](crate::val::Value) from a boolean value
/// ```
/// use libhaystack::val::*;
///
/// let val = Value::from(true);
/// assert!(val.is_bool());
/// // Get the bool value
/// assert!(bool::try_from(&val).unwrap());
/// // Get the Bool value
/// let false_val = Value::from(false);
/// assert!(!Bool::try_from(&false_val).unwrap().value)
///```
///
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
pub struct Bool {
    pub value: bool,
}
// Make a Haystack `Bool` from a bool
impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool { value }
    }
}

// Make a bool from Haystack `Bool`
impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.value
    }
}

/// Converts from bool to a `Bool` `Value`
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(Bool::from(value))
    }
}

/// Converts from `Bool` to a `Bool` `Value`
impl From<Bool> for Value {
    fn from(value: Bool) -> Self {
        Value::Bool(value)
    }
}

/// Tries to convert from `Bool` `Value` to a `bool`
impl TryFrom<&Value> for bool {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.value),
            _ => Err("Value is not an `Bool`"),
        }
    }
}

/// Tries to convert from `Bool` `Value` to a `Bool`
impl TryFrom<&Value> for Bool {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(*v),
            _ => Err("Value is not an `Bool`"),
        }
    }
}
