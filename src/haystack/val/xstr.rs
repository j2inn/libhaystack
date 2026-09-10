// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack XStr

use crate::haystack::val::{ConversionError, Value};

/// XStr with type and value
///
/// # Example
/// Create `XStr` value
/// ```
/// use libhaystack::val::*;
///
/// // Create `XStr`
/// let xstr_value = Value::from(XStr::make("Type", r#"{"test":"100"}"#));
/// assert!(xstr_value.is_xstr());
///
/// // Get the XStr value from the Value
/// assert_eq!(XStr::try_from(&xstr_value).unwrap(), XStr::make("Type", r#"{"test":"100"}"#));
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug, Default)]
pub struct XStr {
    r#type: Box<str>,
    value: Box<str>,
}
impl XStr {
    pub fn make(r#type: &str, value: &str) -> XStr {
        XStr {
            r#type: r#type.into(),
            value: value.into(),
        }
    }

    /// Get the `type` field as a `&str` slice
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// Get the `value` field as a `&str` slice
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Consumes the `XStr` and returns the underlying `type` and `value` as a tuple.
    pub fn into_inner(self) -> (Box<str>, Box<str>) {
        (self.r#type, self.value)
    }
}

/// Converts from `XStr` to a `XStr` `Value`
impl From<XStr> for Value {
    fn from(value: XStr) -> Self {
        Value::XStr(value)
    }
}

/// Tries to convert from `Value` to a `XStr`
impl TryFrom<&Value> for XStr {
    type Error = ConversionError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::XStr(v) => Ok(v.clone()),
            _ => Err("Value is not an `XStr`"),
        }
    }
}
