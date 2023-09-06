// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Coord

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_coord_make_value() {
    let coord = Coord::make(34.0522, 118.2437);

    assert_eq!(coord.lat, 34.0522);
    assert_eq!(coord.long, 118.2437);

    let value: Value = coord.into();

    assert!(value.is_coord());

    assert_eq!(Coord::try_from(&value), Ok(Coord::make(34.0522, 118.2437)))
}

#[test]
fn test_coord_make_invalid_value() {
    let value: Value = Value::from(Coord::make(34.0522, 118.2437));

    assert!(!value.is_number());

    assert_eq!(Number::try_from(&value), Err("Value is not a `Number`"))
}

#[test]
fn test_coord_cmp() {
    let c1: Value = Value::from(Coord::make(34.0522, 118.2437));
    let c2: Value = Value::from(Coord::make(34.0522, 118.2437));

    assert_eq!(c1, c2);

    assert!(c1 >= c2);

    let c2: Value = Value::from(Coord::make(31.0522, 118.2437));
    assert!(c1 >= c2);

    let c2: Value = Value::from(Coord::make(31.0522, 123.2437));
    assert!(c1 >= c2);

    let c2: Value = Value::from(Coord::make(44.0522, 123.2437));
    assert!(c2 > c1);
}
