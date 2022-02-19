// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Remove

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_remove_encode() {
    let value = Value::Remove;
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "R");
}
#[test]
fn test_zinc_remove_decode() {
    let value: Value = from_str("R").unwrap();
    assert_eq!(value, Value::Remove);
}
