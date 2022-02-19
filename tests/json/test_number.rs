// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Number

use libhaystack::units::get_unit_or_default;
#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_serialize_num() {
    let mut value = Value::Number(Number {
        value: 100.0,
        unit: None,
    });

    let mut json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), "100");

    value = Value::make_number_unit(42.0, get_unit_or_default("m/s"));

    json = serde_json::to_string(&value);
    assert_eq!(
        json.unwrap(),
        r#"{"_kind":"number","val":42.0,"unit":"m/s"}"#
    );
}

#[test]
fn test_json_decode_number() {
    let mut value: Value = serde_json::from_str(r#"22.44"#).unwrap();
    assert_eq!(value, Value::make_number(22.44));

    value = serde_json::from_str(r#"91"#).unwrap();
    assert_eq!(value, Value::make_number(91.0));

    value = serde_json::from_str(r#"{"_kind":"number","val":42.0,"unit":"m/s"}"#).unwrap();
    assert_eq!(
        value,
        Value::make_number_unit(42.0, get_unit_or_default("m/s"))
    );
}

#[test]
fn test_json_decode_malformed_number() {
    assert!(serde_json::from_str::<Value>(r#"22-44"#).is_err());

    assert!(
        serde_json::from_str::<Value>(r#"{"_kind":"number","badName":42.0,"unit":"m/s"}"#).is_err()
    );
}
