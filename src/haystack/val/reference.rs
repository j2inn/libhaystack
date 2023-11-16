// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Ref

use crate::haystack::val::Value;
use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::fmt;
use std::hash::Hash;
use uuid::Uuid;

/// Haystack `Ref`
///
/// # Example
/// Create `Ref` value
/// ```
/// use libhaystack::val::*;
///
/// // Create ref from `&str` primitive
/// let ref_value = Value::from(Ref::from("exampleRef"));
/// assert!(ref_value.is_ref());
/// // Ref with display
/// let ref_dis_value = Value::from(Ref{ value: String::from("myRef"), dis: Some(String::from("sample ref")) });
/// assert_eq!(Ref::try_from(&ref_dis_value).unwrap().dis, Some(String::from("sample ref")));
///```
#[derive(Eq, Clone, Debug, Default)]
pub struct Ref {
    pub value: String,
    pub dis: Option<String>,
}

impl Ref {
    /// Make a Ref
    pub fn make(val: &str, dis: Option<&str>) -> Self {
        Ref {
            value: val.into(),
            dis: dis.map(|s| s.into()),
        }
    }

    /// Generate a new Ref based on a V4 UUID
    pub fn gen() -> Ref {
        let uuid = Uuid::new_v4().as_simple().to_string();
        Ref {
            value: format!("{start}-{end}", start = &uuid[0..8], end = &uuid[26..]),
            dis: None,
        }
    }
}

/// Implement equality operator for Ref
impl PartialEq for Ref {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Implement partial ordering used in sorting for Ref
#[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
impl PartialOrd for Ref {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// Implement ordering used in sorting for Ref
impl Ord for Ref {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.value)
    }
}

// Make a Haystack `Ref` from a string value
impl From<&str> for Ref {
    fn from(value: &str) -> Self {
        Ref {
            value: String::from(value),
            dis: None,
        }
    }
}

/// Converts from `Ref` to a `Ref` `Value`
impl From<Ref> for Value {
    fn from(value: Ref) -> Self {
        Value::Ref(value)
    }
}

/// Tries to convert from `Value` to a `Ref`
impl TryFrom<&Value> for Ref {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Ref(v) => Ok(v.clone()),
            _ => Err("Value is not an `Ref`"),
        }
    }
}

impl Hash for Ref {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
