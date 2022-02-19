// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack List

use crate::haystack::val::Value;
use std::vec::Vec;

/// A Haystack List of `Value`s
///
/// # Example
/// Create `List`
/// ```
/// use libhaystack::val::*;
///
/// let list_value = Value::from(vec![Value::from(true), Value::from("A string")]);
/// assert!(list_value.is_list());
/// assert_eq!(List::try_from(&list_value).unwrap().len(), 2);
///```
///
pub type List = Vec<Value>;

/// Converts from `List` to a `List` `Value`
impl From<List> for Value {
    fn from(value: List) -> Self {
        Value::List(value)
    }
}

/// Tries to convert from `Value` to a `List`
impl TryFrom<&Value> for List {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(v) => Ok(v.clone()),
            _ => Err("Value is not an `List`"),
        }
    }
}
