// Copyright (C) 2020 - 2022, J2 Innovations

//! Test NA

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_na_encode() {
    let value = Value::Na;
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"na"}"#);
}

#[test]
fn test_json_na_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"na"}"#).unwrap();
    assert_eq!(value, Value::Na);
}
