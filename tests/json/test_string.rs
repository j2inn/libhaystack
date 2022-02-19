// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Str

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_str_encode() {
    let value = Value::make_str("Foo");
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#""Foo""#);
}

#[test]
fn test_json_str_decode() {
    let value: Value = serde_json::from_str(r#""a string value""#).unwrap();
    assert_eq!(value, Value::make_str("a string value"));

    assert!(serde_json::from_str::<Value>(r#""non terminated"#).is_err());
}
