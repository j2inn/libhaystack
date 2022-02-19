// Copyright (C) 2020 - 2022, J2 Innovations

//! Test List

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_list_encode() {
    let value = Value::make_list(vec![
        Value::make_marker(),
        Value::make_bool(true),
        Value::make_str("foo"),
        Value::make_list(vec![Value::make_number(42.0)]),
    ]);

    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"[{"_kind":"marker"},true,"foo",[42]]"#);
}
#[test]
fn test_json_list_decode() {
    let value: Value = serde_json::from_str(r#"[{"_kind":"marker"},true,"foo",[42.0]]"#).unwrap();
    let list = Value::make_list(vec![
        Value::make_marker(),
        Value::make_bool(true),
        Value::make_str("foo"),
        Value::make_list(vec![Value::make_number(42.0)]),
    ]);
    assert_eq!(value, list);
}

#[test]
fn test_json_decode_malformed_list() {
    assert!(serde_json::from_str::<Value>(r#"[,,"foo",[42.0]]"#).is_err());
}
