// Copyright (C) 2020 - 2022, J2 Innovations

//! Test DateTime

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_datetime_make_value() {
    let datetime =
        DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23-07:00", "Los_Angeles")
            .expect("DateTime");

    assert_eq!(datetime.to_rfc3339(), "2021-06-19T19:48:23-07:00");

    let value: Value = datetime.into();

    assert!(value.is_datetime());

    assert_eq!(
        DateTime::try_from(&value),
        Ok(
            DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23-07:00", "Los_Angeles")
                .unwrap()
        )
    );

    assert!(DateTime::parse_from_rfc3339("2021-06-1919:48:23-10:00").is_err());
}

#[test]
fn test_datetime_from_str_value() {
    let datetime = DateTime::parse_from_rfc3339("2021-06-19T19:48:23-00:00").expect("DateTime");

    assert!(datetime.is_utc());

    assert_eq!(datetime.to_string(), "2021-06-19T19:48:23UTC");
}
