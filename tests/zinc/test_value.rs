// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Value type

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_null_encode() {
    let value = Value::default();
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "N");
}

#[test]
fn test_zinc_null_decode() {
    let value: Value = from_str("N").unwrap();
    assert_eq!(value, Value::default());
}

#[test]
fn test_zinc_null_decode_bad() {
    let value = from_str("O");
    assert!(value.is_err(), "Expecting error");
}
