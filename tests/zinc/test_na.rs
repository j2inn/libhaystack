// Copyright (C) 2020 - 2022, J2 Innovations

//! Test NA

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_na_encode() {
    let value = Value::Na;
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "NA");
}

#[test]
fn test_zinc_na_decode() {
    let value: Value = from_str("NA").unwrap();
    assert_eq!(value, Value::Na);
}

#[test]
fn test_zinc_na_bad() {
    let value = from_str("Nx");
    assert!(value.is_err(), "Expecting error");
}
