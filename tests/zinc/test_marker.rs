// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Marker

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_marker_encode() {
    let value = Value::Marker;
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "M");
}

#[test]
fn test_zinc_marker_decode() {
    let value: Value = from_str("M").unwrap();
    assert_eq!(value, Value::Marker);
}
