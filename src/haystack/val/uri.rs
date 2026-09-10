// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Uri

use crate::haystack::val::{ConversionError, Value};

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
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug, Default)]
pub struct Uri {
    value: Box<str>,
}

impl Uri {
    /// Make a [Uri](crate::val::Uri) from a `&str`
    pub fn make(val: &str) -> Self {
        Uri { value: val.into() }
    }

    /// Get a `&str` slice of the underlying payload
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Consumes the `Uri` and returns the underlying `Box<str>` value.
    pub fn into_inner(self) -> Box<str> {
        self.value
    }
}

// Make a Haystack `Uri` from a String value
impl From<String> for Uri {
    fn from(value: String) -> Self {
        Uri {
            value: value.into(),
        }
    }
}

// Make a Haystack `Uri` from a string value
impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Uri {
            value: value.into(),
        }
    }
}

// Make a Haystack `Uri` from a Box<str> value
impl From<Box<str>> for Uri {
    fn from(value: Box<str>) -> Self {
        Uri { value }
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
    type Error = ConversionError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uri(v) => Ok(v.clone()),
            _ => Err("Value is not an `Uri`"),
        }
    }
}
