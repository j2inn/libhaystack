// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Marker

use crate::haystack::val::Value;

/// Haystack `Marker`
///
/// # Example
/// Create `Marker` value
/// ```
/// use libhaystack::val::*;
///
/// let marker_value = Value::from(Marker);
/// assert!(marker_value.is_marker());
///
/// assert_eq!(Marker::try_from(&marker_value), Ok(Marker));
///```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
pub struct Marker;

/// Converts from Marker to a `Marker` `Value`
impl From<Marker> for Value {
    fn from(_: Marker) -> Self {
        Value::Marker
    }
}

/// Tries to convert from `Marker` `Value` to a `Marker`
impl TryFrom<&Value> for Marker {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Marker => Ok(Marker),
            _ => Err("Value is not an `Marker`"),
        }
    }
}
