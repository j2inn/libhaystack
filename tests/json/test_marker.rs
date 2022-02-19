// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Marker

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_marker_encode() {
    let value = Value::Marker;
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"marker"}"#);
}

#[test]
fn test_json_marker_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"marker"}"#).unwrap();
    assert_eq!(value, Value::Marker);
}
