// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Bool JSON
//!
#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_bool_encode() {
    let mut value = Value::Bool(Bool::from(true));
    let mut json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), "true");

    value = Value::Bool(Bool::from(false));
    json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), "false");
}

#[test]
fn test_json_bool_decode() {
    let value: Value = serde_json::from_str(r#"true"#).unwrap();
    assert_eq!(value, Value::Bool(Bool::from(true)));

    let value: Value = serde_json::from_str(r#"false"#).unwrap();
    assert_eq!(value, Value::Bool(Bool::from(false)));

    assert!(serde_json::from_str::<Value>(r#"fal"#).is_err());
}

#[test]
fn test_json_decode_malformed_bool() {
    assert!(serde_json::from_str::<Value>(r#"true/"#).is_err());
}
