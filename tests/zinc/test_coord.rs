// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Coord

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_coord_encode() {
    let value = Coord {
        lat: 45.0,
        long: 23.0,
    };

    let zinc = value.to_zinc_string();
    assert_eq!(zinc, Ok("C(45,23)".into()));
    assert_eq!(Value::from(value).to_zinc_string(), Ok("C(45,23)".into()));
}

#[test]
fn test_zinc_coord_decode() {
    let value = from_str("C(45.0,23.0)");
    assert_eq!(value.ok(), Some(Value::make_coord_from(45.0, 23.0)));

    let value = from_str("C(-45.0,-23.0)");
    assert_eq!(value.ok(), Some(Value::make_coord_from(-45.0, -23.0)));

    let value = from_str("C(45.0)");
    assert!(value.is_err());
}

#[test]
fn test_zinc_coord_decode_bad() {
    assert!(from_str("C(45,23").is_err());
}
