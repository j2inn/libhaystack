// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Uri

use crate::haystack::val::Value;

/// Haystack `Uri`
///
/// # Example
/// Create `Uri` value
/// ```
/// use libhaystack::val::*;
///
/// // Create `Uri` from `&str` primitive
/// let uri_value = Value::from(Uri::from("/an/uri"));
/// assert!(uri_value.is_uri());
///
/// // Get the uri value from the Value
/// assert_eq!(Uri::try_from(&uri_value).unwrap(), Uri::from("/an/uri"));
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
pub struct Uri {
    pub value: String,
}

impl Uri {
    /// Make a [Uri](crate::val::Uri) from a `&str`
    pub fn make(val: &str) -> Self {
        Uri { value: val.into() }
    }
}

// Make a Haystack `Uri` from a string value
impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Uri {
            value: String::from(value),
        }
    }
}

/// Converts from `Uri` to a `Uri` `Value`
impl From<Uri> for Value {
    fn from(value: Uri) -> Self {
        Value::Uri(value)
    }
}

/// Tries to convert from `Value` to a `Uri`
impl TryFrom<&Value> for Uri {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uri(v) => Ok(v.clone()),
            _ => Err("Value is not an `Uri`"),
        }
    }
}
