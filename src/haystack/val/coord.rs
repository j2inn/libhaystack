// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Coord

use crate::haystack::val::Value;
use std::{
    cmp::Ordering,
    convert::{From, TryFrom},
    hash::Hash,
};

/// Coordinate latitude and longitude
////
/// # Example
/// Create a coord value
/// ```
/// use libhaystack::val::*;
///
/// let coord = Value::from(Coord {
///             lat: 45.0,
///             long: 23.0,
/// });
/// assert!(coord.is_coord());
/// // Get the Coord value
/// assert_eq!(Coord::try_from(&coord).unwrap().lat, 45.0);
///```
///
#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub lat: f64,
    pub long: f64,
}
impl Coord {
    pub fn make(lat: f64, long: f64) -> Coord {
        Coord { lat, long }
    }
}

/// Converts from `Coord` to a `Coord` `Value`
impl From<Coord> for Value {
    fn from(value: Coord) -> Self {
        Value::Coord(value)
    }
}

/// Tries to convert from `Value` to a `Coord`
impl TryFrom<&Value> for Coord {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Coord(v) => Ok(*v),
            _ => Err("Value is not an `Coord`"),
        }
    }
}

impl Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lat.to_bits().hash(state);
        self.long.to_bits().hash(state);
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.lat.eq(&other.lat) && self.long.eq(&other.long)
    }
}

impl Eq for Coord {}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.lat
                .to_bits()
                .cmp(&other.lat.to_bits())
                .cmp(&self.long.to_bits().cmp(&other.long.to_bits())),
        )
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lat
            .to_bits()
            .cmp(&other.lat.to_bits())
            .cmp(&self.long.to_bits().cmp(&other.long.to_bits()))
    }
}
