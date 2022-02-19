// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Remove

use crate::haystack::val::Value;

/// Haystack `Remove`
///
/// # Example
/// Create `Remove` value
/// ```
/// use libhaystack::val::*;
///
/// let remove_value = Value::from(Remove);
/// assert!(remove_value.is_remove());
///
/// assert_eq!(Remove::try_from(&remove_value), Ok(Remove));
///```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
pub struct Remove;

/// Converts from Remove to a `Remove` `Value`
impl From<Remove> for Value {
    fn from(_: Remove) -> Self {
        Value::Remove
    }
}

/// Tries to convert from `Remove` `Value` to a `Remove`
impl TryFrom<&Value> for Remove {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Remove => Ok(Remove),
            _ => Err("Value is not an `Remove`"),
        }
    }
}
