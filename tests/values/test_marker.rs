// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Marker

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_marker_from_value() {
    let value: Value = Marker.into();

    assert!(value.is_marker());

    assert_eq!(Marker::try_from(&value), Ok(Marker));
}

#[test]
fn test_marker_value() {
    let value = Value::Marker;
    assert!(value.is_marker());
    assert!(!value.is_bool());

    assert!(Value::make_marker().is_marker());
}
