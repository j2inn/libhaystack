// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Number

use crate::{haystack::val::Value, units::Unit, units::DEFAULT_UNIT};
use std::{
    cmp::Ordering,
    convert::{From, TryFrom},
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

/// Haystack `Number` floating point value and optional unit
///
/// # Example
/// Create `Number` value
/// ```
/// use libhaystack::val::*;
/// use libhaystack::units::Unit;
/// use libhaystack::units::{get_unit, get_unit_or_default};
///
/// // Create number from `f64` primitive
/// let number_value = Value::from(42.0);
/// assert!(number_value.is_number());
/// assert_eq!(f64::try_from(&number_value), Ok(42.0));
///
/// // Create `Number` from `i32` primitive
/// let num = Number::from(100);
/// assert_eq!(num.value, 100.0);
/// assert!(Value::from(num).is_number());
///
/// // Number with unit
/// let num_unit_value = Value::from(Number::make_with_unit(100.0, "sec".into()));
/// assert_eq!(Number::try_from(&num_unit_value).unwrap().unit, get_unit("sec"));
///```
#[derive(Copy, Clone, Debug, Default)]
pub struct Number {
    pub value: f64,
    pub unit: Option<&'static Unit>,
}

impl Number {
    /// Makes number without unit
    pub fn make(value: f64) -> Number {
        Number { value, unit: None }
    }

    /// Makes number with specified unit
    pub fn make_with_unit(value: f64, unit: &'static Unit) -> Number {
        Number {
            value,
            unit: if unit != &*DEFAULT_UNIT {
                Some(unit)
            } else {
                None
            },
        }
    }
}

// Make a Haystack `Number` from a float value
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number { value, unit: None }
    }
}
// Make a Haystack `Number` from an int value
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number {
            value: value as f64,
            unit: None,
        }
    }
}

/// Converts from `Number` to a `Number` `Value`
impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Value::Number(value)
    }
}

/// Converts from `f64` to a `Number` `Value`
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(Number::from(value))
    }
}

/// Converts from `i32` to a `Number` `Value`
impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(Number::from(value))
    }
}

/// Tries to convert from `Number` `Value` to a `f64`
impl TryFrom<&Value> for f64 {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(v) => Ok(v.value),
            _ => Err("Value is not a `Number`"),
        }
    }
}

/// Tries to convert from `Number` `Value` to a `Number`
impl TryFrom<&Value> for Number {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(v) => Ok(*v),
            _ => Err("Value is not a `Number`"),
        }
    }
}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
        self.unit.hash(state);
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}

impl Eq for Number {}

#[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit == other.unit {
            self.value.partial_cmp(&other.value)
        } else {
            None
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value == other.value {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

/// Addition operator
impl Add<Number> for Number {
    type Output = Result<Number, String>;

    fn add(self, other: Number) -> Self::Output {
        let unit = if self.unit == other.unit {
            self.unit
        } else if self.unit.is_none() {
            other.unit
        } else if other.unit.is_none() {
            self.unit
        } else {
            return Err(format!(
                "Invalid addition units: {this:?}, {other:?}",
                this = self.unit,
                other = other.unit
            ));
        };

        Ok(Number::make_with_unit(
            self.value + other.value,
            unit.unwrap_or(&*DEFAULT_UNIT),
        ))
    }
}

/// Subtraction operator
impl Sub<Number> for Number {
    type Output = Result<Number, String>;

    fn sub(self, other: Number) -> Self::Output {
        let unit = if self.unit == other.unit {
            self.unit
        } else if self.unit.is_none() {
            other.unit
        } else if other.unit.is_none() {
            self.unit
        } else {
            return Err(format!(
                "Invalid subtraction units: {this:?}, {other:?}",
                this = self.unit,
                other = other.unit
            ));
        };

        Ok(Number::make_with_unit(
            self.value - other.value,
            unit.unwrap_or(&*DEFAULT_UNIT),
        ))
    }
}

/// Multiplication operator
impl Mul<Number> for Number {
    type Output = Result<Number, String>;

    fn mul(self, other: Number) -> Self::Output {
        let unit = if self.unit.is_none() {
            other.unit
        } else if other.unit.is_none() {
            self.unit
        } else {
            match self.unit.unwrap_or(&*DEFAULT_UNIT) * other.unit.unwrap_or(&*DEFAULT_UNIT) {
                Ok(u) => Some(u),
                Err(err) => return Err(err),
            }
        };

        Ok(Number::make_with_unit(
            self.value * other.value,
            unit.unwrap_or(&*DEFAULT_UNIT),
        ))
    }
}

/// Division operator
impl Div<Number> for Number {
    type Output = Result<Number, String>;

    fn div(self, other: Number) -> Self::Output {
        let unit = if self.unit.is_none() {
            other.unit
        } else if other.unit.is_none() {
            self.unit
        } else {
            match self.unit.unwrap_or(&*DEFAULT_UNIT) / other.unit.unwrap_or(&*DEFAULT_UNIT) {
                Ok(u) => Some(u),
                Err(err) => return Err(err),
            }
        };

        Ok(Number::make_with_unit(
            self.value / other.value,
            unit.unwrap_or(&*DEFAULT_UNIT),
        ))
    }
}
