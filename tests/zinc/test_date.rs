// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Date
#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_date_encode() {
    let date = Date::from_ymd(2021, 6, 18).expect("Date");

    assert_eq!(date.to_zinc_string(), Ok("2021-06-18".into()));
    assert_eq!(Value::from(date).to_zinc_string(), Ok("2021-06-18".into()));
}

#[test]
fn test_zinc_date_decode() {
    assert_eq!(
        from_str("2021-06-18").ok(),
        Some(Date::from_ymd(2021, 6, 18).expect("Date").into())
    );

    assert!(from_str("2021-100-18").is_err());
    assert!(from_str("2021-").is_err());
    assert!(from_str("2021-100-").is_err());
}

#[test]
fn test_zinc_date_decode_bad() {
    assert!(from_str("2020-03|22").is_err());
}
