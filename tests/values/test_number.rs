// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Number

use std::f64::NAN;

use libhaystack::units::get_unit_or_default;
#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_number_make_value() {
    let number: Number = Number::make(41.0);

    let value: Value = number.into();

    assert!(value.is_number());
    assert!(!value.is_bool());

    assert_eq!(Number::try_from(&value), Ok(Number::make(41.0)));
}

#[test]
fn test_number_make() {
    let number: Number = Number::make_with_unit(42.0, get_unit_or_default("s"));

    assert_eq!(number.value, 42.0);
    assert_eq!(number.unit, Some(get_unit_or_default("s")));
}

#[test]
fn test_number_from() {
    let number: Number = 10.into();

    assert_eq!(number.value, 10.0);
    assert_eq!(number.unit, None);

    let number: Number = 100.9.into();

    assert_eq!(number.value, 100.9);
    assert_eq!(number.unit, None);
}

#[test]
fn test_number_addition() {
    let a: Number = 10.into();
    let b: Number = 1.into();

    assert_eq!(a + b, Ok(11.into()));

    let a: Number = 10.into();
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert_eq!(
        a + b,
        Ok(Number::make_with_unit(30.0, get_unit_or_default("ft")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));

    assert_eq!(
        a + b,
        Ok(Number::make_with_unit(40.0, get_unit_or_default("m")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert!((a + b).is_err());
}

#[test]
fn test_number_subtraction() {
    let a: Number = 10.into();
    let b: Number = 1.into();

    assert_eq!(a - b, Ok(9.into()));

    let a: Number = 10.into();
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert_eq!(
        a - b,
        Ok(Number::make_with_unit(-10.0, get_unit_or_default("ft")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));

    assert_eq!(
        a - b,
        Ok(Number::make_with_unit(0.0, get_unit_or_default("m")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert!((a - b).is_err());
}

#[test]
fn test_number_multiplication() {
    let a: Number = 10.into();
    let b: Number = 2.into();

    assert_eq!(a * b, Ok(20.into()));

    let a: Number = 10.into();
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert_eq!(
        a * b,
        Ok(Number::make_with_unit(200.0, get_unit_or_default("ft")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("megawatt"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("hour"));

    assert_eq!(
        a * b,
        Ok(Number::make_with_unit(
            400.0,
            get_unit_or_default("megawatt_hour")
        ))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert!((a * b).is_err());
}

#[test]
fn test_number_division() {
    let a: Number = 10.into();
    let b: Number = 2.into();

    assert_eq!(a / b, Ok(5.into()));

    let a: Number = 10.into();
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert_eq!(
        a / b,
        Ok(Number::make_with_unit(0.5, get_unit_or_default("ft")))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("kg"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("hour"));

    assert_eq!(
        a / b,
        Ok(Number::make_with_unit(
            1.0,
            get_unit_or_default("kilograms_per_hour")
        ))
    );

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("ft"));

    assert!((a * b).is_err());
}

#[test]
fn test_number_eq() {
    let a: Number = 10.into();
    let b: Number = 10.into();
    assert_eq!(a, b);

    let a: Number = NAN.into();
    let b: Number = NAN.into();
    assert_ne!(a, b);

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    assert_eq!(a, b);

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make(20.0);
    assert_ne!(a, b);
}

#[test]
fn test_number_cmp() {
    let a: Number = 20.into();
    let b: Number = (-10).into();
    assert!(a > b);

    let a: Number = 10.into();
    let b: Number = 10.into();
    assert!(a >= b);

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(-20.0, get_unit_or_default("m"));
    assert!(a > b);

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    assert!(a >= b);

    let a: Number = Number::make_with_unit(-20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(-20.0, get_unit_or_default("m"));
    assert!(a <= b);

    let a: Number = Number::make_with_unit(-20.0, get_unit_or_default("m"));
    let b: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    assert!(a < b);

    let a: Number = Number::make_with_unit(20.0, get_unit_or_default("m"));
    let b: Number = Number::make(20.0);
    // Because units are different they can't be compared
    assert!(!(a > b));
    assert!(!(a >= b));
    assert!(!(a == b));
    assert!(a != b);
    assert!(!(a < b));
    assert!(!(a <= b));

    let a: Number = NAN.into();
    let b: Number = NAN.into();
    assert!(a > b);
    assert!(a >= b);
}
