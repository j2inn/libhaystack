// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Date

#[cfg(test)]
use libhaystack::val::*;
use std::str::FromStr;

#[test]
fn test_date_make_value() {
    let date = Date::from_ymd(2021, 6, 18).expect("Date");

    assert_eq!(date.to_string(), "2021-06-18");

    let value: Value = date.into();

    assert!(value.is_date());

    assert_eq!(
        Date::try_from(&value),
        Ok(Date::from_ymd(2021, 6, 18).unwrap())
    );

    assert!(Date::from_ymd(2021, 60, 18).is_err());
}

#[test]
fn test_date_from_str_value() {
    let date = Date::from_str("2021-06-18").expect("Date");

    assert_eq!(date.to_string(), "2021-06-18");

    assert!(Date::from_str("2021-06999").is_err());
}
