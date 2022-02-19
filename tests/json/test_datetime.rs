// Copyright (C) 2020 - 2022, J2 Innovations

//! Test DateTime

#[cfg(test)]
use chrono_tz::OffsetName;
use libhaystack::val::*;

#[test]
fn test_json_datetime_encode() {
    let value = Value::make_datetime_from_iso("1996-12-19T16:39:57-08:00").unwrap();
    let json = serde_json::to_string(&value);
    assert_eq!(
        json.unwrap(),
        r#"{"_kind":"dateTime","val":"1996-12-19T16:39:57-08:00","tz":"GMT+8"}"#
    );

    let value = Value::make_datetime_from_iso("2020-09-18T22:44:51.716Z").unwrap();
    let json = serde_json::to_string(&value);
    assert_eq!(
        json.unwrap(),
        r#"{"_kind":"dateTime","val":"2020-09-18T22:44:51.716Z"}"#
    );
}

#[test]
fn test_json_datetime_decode() {
    let value: Value =
        serde_json::from_str(r#"{"_kind":"dateTime","val":"1996-12-19T16:39:57-08:00"}"#).unwrap();
    assert_eq!(
        value,
        Value::make_datetime_from_iso("1996-12-19T16:39:57-08:00").unwrap()
    );

    match value {
        Value::DateTime(dt) => {
            let tz: String = dt.offset().tz_id().to_string();
            assert_eq!(tz, "Etc/GMT+8", "Timezone not found")
        }
        _ => panic!("Not a date time"),
    }
}

#[test]
fn test_json_datetime_tz_decode() {
    let value: Value = serde_json::from_str(
        r#"{"_kind":"dateTime","val":"2020-12-15T08:00:00-07:00","tz":"Phoenix"}"#,
    )
    .unwrap();
    assert_eq!(
        value,
        Value::make_datetime_from_iso("2020-12-15T08:00:00-07:00").unwrap()
    );

    match value {
        Value::DateTime(dt) => {
            let tz: String = dt.offset().tz_id().to_string();
            assert!(tz.contains("Phoenix"), "Timezone not found")
        }
        _ => panic!("Not a date time"),
    }
}

#[test]
fn test_json_decode_malformed_datetime() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"dateTime","val":"1996-12-19T"}"#).is_err());
}
