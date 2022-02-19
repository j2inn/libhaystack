// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack NA

use crate::haystack::val::Value;

/// Haystack `NA`
///
/// # Example
/// Create `Na` value
/// ```
/// use libhaystack::val::*;
///
/// let na_value = Value::from(Na);
/// assert!(na_value.is_na());
///
/// assert_eq!(Na::try_from(&na_value), Ok(Na));
///```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
pub struct Na;

/// Converts from Na to a `Na` `Value`
impl From<Na> for Value {
    fn from(_: Na) -> Self {
        Value::Na
    }
}

/// Tries to convert from `Na` `Value` to a `Na`
impl TryFrom<&Value> for Na {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Na => Ok(Na),
            _ => Err("Value is not an `NA`"),
        }
    }
}
