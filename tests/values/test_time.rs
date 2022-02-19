// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Time

#[cfg(test)]
use libhaystack::val::*;
use std::str::FromStr;

#[test]
fn test_time_make_value() {
    let time = Time::from_hms(20, 6, 18).expect("Time");

    assert_eq!(time.to_string(), "20:06:18");

    let value: Value = time.into();

    assert!(value.is_time());

    assert_eq!(
        Time::try_from(&value),
        Ok(Time::from_hms(20, 6, 18).unwrap())
    );

    assert!(Time::from_hms(100, 6, 18).is_err());

    let time = Time::from_hms_milli(20, 6, 18, 999).expect("Time");

    assert_eq!(time.to_string(), "20:06:18.999");
}

#[test]
fn test_time_from_str_value() {
    let time = Time::from_str("20:06:18").expect("Time");

    assert_eq!(time.to_string(), "20:06:18");

    assert!(Time::from_str("100:34").is_err());
}
