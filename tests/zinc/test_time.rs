// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Time

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_time_encode() {
    let value = Value::make_time(Time::from_hms(17, 25, 33).expect("Time"));
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "17:25:33");
}

#[test]
fn test_zinc_time_decode() {
    let value: Value = from_str("17:25:33").unwrap();
    assert_eq!(
        value,
        Value::make_time(Time::from_hms(17, 25, 33).expect("Time"))
    );
}

#[test]
fn test_zinc_time_millis_decode() {
    let value: Value = from_str("17:25:33.999").unwrap();
    assert_eq!(
        value,
        Value::make_time(Time::from_hms_milli(17, 25, 33, 999).expect("Time"))
    );
}

#[test]
fn test_zinc_time_decode_bad() {
    assert!(from_str("17-25:33").is_err());
}
