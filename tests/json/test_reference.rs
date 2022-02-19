// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Ref

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_ref_encode() {
    let mut value = Value::Ref(Ref {
        value: String::from("someId"),
        dis: Some(String::from("dis")),
    });

    let mut json = serde_json::to_string(&value);
    assert_eq!(
        json.unwrap(),
        r#"{"_kind":"ref","val":"someId","dis":"dis"}"#
    );

    value = Value::make_ref("id");
    json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"ref","val":"id"}"#);
}

#[test]
fn test_json_ref_decode() {
    let mut value: Value =
        serde_json::from_str(r#"{"_kind":"ref","val":"someId","dis":"dis"}"#).unwrap();
    assert_eq!(
        value,
        Value::Ref(Ref {
            value: String::from("someId"),
            dis: Some(String::from("dis")),
        })
    );

    value = serde_json::from_str(r#"{"_kind":"ref","val":"id"}"#).unwrap();

    assert_eq!(value, Value::make_ref("id"));
}

#[test]
fn test_json_decode_malformed_ref() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"ref","val":1}"#).is_err());
}
