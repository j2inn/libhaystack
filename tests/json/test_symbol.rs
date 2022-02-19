// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Symbol

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_symbol_encode() {
    let value = Value::make_symbol("symbol");
    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"symbol","val":"symbol"}"#);
}

#[test]
fn test_json_symbol_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"symbol","val":"symbol"}"#).unwrap();
    assert_eq!(value, Value::make_symbol("symbol"));
}

#[test]
fn test_json_decode_malformed_symbol() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"symbol","valz":1}"#).is_err());
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"symbol","val":w}"#).is_err());
}
