// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Dict

#[cfg(test)]
use libhaystack::dict;
use libhaystack::val::*;

#[test]
fn test_json_dict_encode() {
    let rec = dict! {
        "site" => Value::make_marker(),
        "name" => Value::make_str("Foo"),
        "dict" => Value::make_dict(dict! {"foo" => Value::make_bool(true)})
    };

    let json = serde_json::to_string(&Value::make_dict(rec));
    assert_eq!(
        json.unwrap(),
        r#"{"dict":{"foo":true},"name":"Foo","site":{"_kind":"marker"}}"#
    );
}
#[test]
fn test_json_dict_decode() {
    let value: Value =
        serde_json::from_str(r#"{"dict":{"foo":true},"name":"Foo","site":{"_kind":"marker"}}"#)
            .unwrap();
    let dict = dict! {
        "site" => Value::make_marker(),
        "name" => Value::make_str("Foo"),
        "dict" => Value::make_dict(dict! {"foo" => Value::make_bool(true)})
    };

    assert_eq!(value, Value::make_dict(dict.clone()));

    let value: Value = serde_json::from_str(
        r#"{"_kind":"dict", "dict":{"foo":true},"name":"Foo","site":{"_kind":"marker"}}"#,
    )
    .unwrap();

    assert_eq!(value, Value::make_dict(dict));
}

#[test]
fn test_json_decode_malformed_dict() {
    assert!(serde_json::from_str::<Value>(r#"{"name":}"#).is_err());
}
