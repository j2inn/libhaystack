// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Value type

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_null_encode() {
    let value = Value::default();
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), "null");
}

#[test]
fn test_json_null_decode() {
    let value: Value = serde_json::from_str("null").unwrap();
    assert_eq!(value, Value::default());
}

#[test]
fn test_json_decode_malformed_null() {
    assert!(serde_json::from_str::<Value>("nul").is_err());
}

#[test]
fn test_json_decode_malformed_kind() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"doobardoodo"}"#).is_err());
}
