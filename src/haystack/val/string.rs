// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Str

use core::fmt;
use std::ops::Deref;

use crate::haystack::val::Value;

/// Haystack `Str`
///
/// # Example
/// Create `Str` value
/// ```
/// use libhaystack::val::*;
///
/// // Create `Str` from `&str` primitive
/// let str_value = Value::from(Str::from("sample string"));
/// assert!(str_value.is_str());
///
/// // Get the `Str` value from the Value
/// assert_eq!(Str::try_from(&str_value).unwrap(), Str::from("sample string"));
/// // Get a std::String from a Value
/// assert_eq!(String::try_from(&str_value).unwrap(), "sample string");
///
/// // Create `Str` from literal
/// let str:Str = "sample string".into();
/// assert_eq!(str.as_str(), "sample string");
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug, Default)]
pub struct Str {
    pub value: String,
}

impl Str {
    /// Constructs a Str
    pub fn make(val: &str) -> Self {
        Str { value: val.into() }
    }

    /// Get a `&str` slice of the underlying `String` payload
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

// Make a Haystack `Str` from a `String`
impl From<String> for Str {
    fn from(value: String) -> Self {
        Str { value }
    }
}

// Make a Haystack `Str` from a `&str` slice
impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Str {
            value: value.to_owned(),
        }
    }
}

/// Converts from `&str` slice to a `Str` `Value`
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.into())
    }
}

/// Converts from `Str` to a `Str` `Value`
impl From<Str> for Value {
    fn from(value: Str) -> Self {
        Value::Str(value)
    }
}

/// Tries to convert from `Str` `Value` to a `String`
impl TryFrom<&Value> for String {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Str(v) => Ok(v.value.clone()),
            _ => Err("Value is not an `Str`"),
        }
    }
}

/// Tries to convert from `Str` `Value` to a `Str`
impl TryFrom<&Value> for Str {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Str(v) => Ok(v.clone()),
            _ => Err("Value is not an `Str`"),
        }
    }
}
/// Formats the `Str` using its inner `String`'s `Display` implementation
impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Dereferences `Str` to `str`, making all `str` methods available on `&Str`
/// and allowing `&Str` to be used where `&str` is expected.
impl Deref for Str {
    type Target = str;
    fn deref(&self) -> &str {
        &self.value
    }
}

/// Allows `Str` to be used with generic APIs that accept `impl AsRef<str>`
impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

/// Extracts the owned `String` from a `Str`
impl From<Str> for String {
    fn from(s: Str) -> String {
        s.value
    }
}

/// Converts an owned `String` directly to a `Str` `Value`
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Str(value.into())
    }
}

/// Allows comparing `Str` with `str` directly: `some_str == "foo"`
impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

/// Allows comparing `Str` with `String` directly: `some_str == owned`
impl PartialEq<String> for Str {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}
