// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Time

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_time_encode() {
    let value = Value::make_time(Time::from_hms(17, 25, 33).expect("Time"));
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"time","val":"17:25:33"}"#);
}

#[test]
fn test_json_time_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"time","val":"17:25:33"}"#).unwrap();
    assert_eq!(
        value,
        Value::make_time(Time::from_hms(17, 25, 33).expect("Time"))
    );
}

#[test]
fn test_json_decode_malformed_time() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"time","val":"17"}"#).is_err());
}
