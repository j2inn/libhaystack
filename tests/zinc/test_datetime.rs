// Copyright (C) 2020 - 2022, J2 Innovations

//! Test DateTime

#[cfg(test)]
use chrono_tz::OffsetName;
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_datetime_encode() {
    let value = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap();
    let zinc = value.to_zinc_string();
    assert_eq!(zinc.unwrap(), "1996-12-19T16:39:57-08:00 GMT+8");

    assert_eq!(
        Value::from(value).to_zinc_string(),
        Ok("1996-12-19T16:39:57-08:00 GMT+8".into())
    );
}

#[test]
fn test_zinc_datetime_utc_decode() {
    let value: Value = from_str("1996-12-19T16:39:57Z   ").unwrap();
    assert_eq!(
        value,
        Value::make_datetime_from_iso("1996-12-19T16:39:57Z").unwrap()
    );
}

#[test]
fn test_zinc_datetime_tz_decode() {
    let value: Value = from_str("2010-03-11T23:55:00-05:00 New_York").unwrap();
    assert_eq!(
        value,
        Value::make_datetime_from_iso("2010-03-11T23:55:00-05:00").unwrap()
    );

    match value {
        Value::DateTime(dt) => {
            assert_eq!(dt.offset().tz_id(), "America/New_York")
        }
        _ => panic!("Not a datetime"),
    }

    let value: Value = from_str("2021-03-11T23:55:00+03:00 Europe/Bucharest").unwrap();
    let dt = DateTime::try_from(&value).expect("DateTime");

    assert_eq!(dt.offset().tz_id(), "Europe/Bucharest");

    let value: Value = from_str("2021-03-11T23:55:00+03:00 Etc/GMT+3").unwrap();
    let dt = DateTime::try_from(&value).expect("DateTime");

    assert_eq!(dt.offset().tz_id(), "Etc/GMT+3");
}

#[test]
fn test_zinc_datetime_utc_decode_millis() {
    assert!(from_str("1996-12-19T16:39:57.2Z").is_ok());
}

#[test]
fn test_zinc_datetime_utc_decode_tz() {
    assert!(from_str("2020-02-10T16:00:00.0Z London").is_ok());
}

#[test]
fn test_zinc_datetime_utc_decode_bad() {
    assert!(from_str("1996-12-19T16:39:57K").is_err());
}

#[test]
fn test_zinc_datetime_tz_decode_bad() {
    assert!(from_str("2010-03-11T23:55:00-05:00").is_err());
}
