// Copyright (C) 2020 - 2022, J2 Innovations

//! Test XStr

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_xstr_encode() {
    let value = Value::make_xstr_from("Foo", "bar");

    let json = serde_json::to_string(&value);
    assert_eq!(
        json.unwrap(),
        r#"{"_kind":"xstr","type":"Foo","val":"bar"}"#
    );
}
#[test]
fn test_json_xstr_decode() {
    let value: Value =
        serde_json::from_str(r#"{"_kind":"xstr","type":"Foo","val":"bar"}"#).unwrap();
    assert_eq!(value, Value::make_xstr_from("Foo", "bar"));
}

#[test]
fn test_json_decode_malformed_xstr() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"xstr","type":true,"val":"bar"}"#).is_err());
}
