// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Uri

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_uri_encode() {
    let value = Value::make_uri("http://zoo.bar");
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"uri","val":"http://zoo.bar"}"#);
}
#[test]
fn test_json_uri_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"uri","val":"http://zoo.bar"}"#).unwrap();
    assert_eq!(value, Value::make_uri("http://zoo.bar"));
}

#[test]
fn test_json_decode_malformed_uri() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"uri"}"#).is_err());
}
