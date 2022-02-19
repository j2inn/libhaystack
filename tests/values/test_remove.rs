// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Remove

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_remove_from_value() {
    let value: Value = Remove.into();

    assert!(value.is_remove());

    assert_eq!(Remove::try_from(&value), Ok(Remove));
}

#[test]
fn test_remove_value() {
    let value = Value::Remove;
    assert!(value.is_remove());
    assert!(!value.is_na());
}
