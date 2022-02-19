// Copyright (C) 2020 - 2022, J2 Innovations

//! Test NA

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_na_from_value() {
    let value: Value = Na.into();

    assert!(value.is_na());

    assert_eq!(Na::try_from(&value), Ok(Na));
}

#[test]
fn test_na_value() {
    let value = Value::Na;
    assert!(value.is_na());
    assert!(!value.is_marker());
}
