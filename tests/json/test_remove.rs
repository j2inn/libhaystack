// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Remove

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_remove_encode() {
    let value = Value::Remove;
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"remove"}"#);
}
#[test]
fn test_json_remove_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"remove"}"#).unwrap();
    assert_eq!(value, Value::Remove);
}
