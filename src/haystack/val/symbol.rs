// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Symbol

use std::fmt;

use crate::haystack::val::{ConversionError, Value};

/// Haystack `Symbol`
///
/// # Example
/// Create `Symbol` value
/// ```
/// use libhaystack::val::*;
///
/// // Create `Symbol` from `&str` primitive
/// let sym_value = Value::from(Symbol::from("symbol"));
/// assert!(sym_value.is_symbol());
///
/// // Get the symbol value from the Value
/// assert_eq!(Symbol::try_from(&sym_value).unwrap(), Symbol::from("symbol"));
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug, Default)]
pub struct Symbol {
    value: Box<str>,
}

impl Symbol {
    /// Make a [Symbol](crate::val::Symbol) from a `&str`
    pub fn make(val: &str) -> Self {
        Symbol { value: val.into() }
    }

    /// Get a `&str` slice of the underlying payload
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Consumes the `Symbol` and returns the underlying `Box<str>` value.
    pub fn into_inner(self) -> Box<str> {
        self.value
    }
}

// Make a Haystack `Symbol` from a string value
impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol {
            value: value.into(),
        }
    }
}

// Make a Haystack `Symbol` from a String value
impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Symbol {
            value: value.into(),
        }
    }
}

// Make a Haystack `Symbol` from a Box<str> value
impl From<Box<str>> for Symbol {
    fn from(value: Box<str>) -> Self {
        Symbol { value }
    }
}

/// Converts from `Symbol` to a `Symbol` `Value`
impl From<Symbol> for Value {
    fn from(value: Symbol) -> Self {
        Value::Symbol(value)
    }
}

/// Tries to convert from `Value` to a `Symbol`
impl TryFrom<&Value> for Symbol {
    type Error = ConversionError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Symbol(v) => Ok(v.clone()),
            _ => Err("Value is not an `Symbol`"),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^{}", self.value)
    }
}
