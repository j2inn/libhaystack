// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Symbol

use crate::haystack::val::Value;

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
    pub value: String,
}

impl Symbol {
    /// Make a [Symbol](crate::val::Symbol) from a `&str`
    pub fn make(val: &str) -> Self {
        Symbol { value: val.into() }
    }
}

// Make a Haystack `Symbol` from a string value
impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol {
            value: String::from(value),
        }
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
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Symbol(v) => Ok(v.clone()),
            _ => Err("Value is not an `Symbol`"),
        }
    }
}
