// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Date
#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_date_encode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"date","val":"2020-03-22"}"#).unwrap();
    assert_eq!(
        value,
        Value::make_date(Date::from_ymd(2020, 3, 22).expect("Date"))
    );
}

#[test]
fn test_json_date_decode() {
    let value = Value::make_date(Date::from_ymd(2020, 3, 22).expect("Date"));
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"date","val":"2020-03-22"}"#);
}

#[test]
fn test_json_decode_malformed_date() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"date","val":"2020-03"}"#).is_err());
}
