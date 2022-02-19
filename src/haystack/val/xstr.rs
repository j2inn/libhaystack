// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack XStr

use crate::haystack::val::Value;

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
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
pub struct XStr {
    pub r#type: String,
    pub value: String,
}
impl XStr {
    pub fn make(r#type: &str, value: &str) -> XStr {
        XStr {
            r#type: String::from(r#type),
            value: String::from(value),
        }
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
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::XStr(v) => Ok(v.clone()),
            _ => Err("Value is not an `XStr`"),
        }
    }
}
