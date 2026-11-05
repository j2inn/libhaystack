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
fn test_json_serialize_num_outside_i64_range() {
    // 2^63 is the first whole value above the i64 range
    for num in [1e300, -1e300, 1e19, -1e19, 9223372036854775808.0, f64::MAX] {
        let json = serde_json::to_string(&Value::make_number(num)).unwrap();
        let decoded: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, Value::make_number(num), "encoded as {json}");
    }
}

#[test]
fn test_json_serialize_num_at_i64_bounds() {
    // -2^63 is exactly i64::MIN
    let mut json = serde_json::to_string(&Value::make_number(i64::MIN as f64));
    assert_eq!(json.unwrap(), i64::MIN.to_string());

    // The largest f64 below 2^63
    json = serde_json::to_string(&Value::make_number(9223372036854774784.0));
    assert_eq!(json.unwrap(), "9223372036854774784");
}

#[test]
fn test_json_serialize_non_finite_num() {
    for (num, val) in [
        (f64::INFINITY, "INF"),
        (f64::NEG_INFINITY, "-INF"),
        (f64::NAN, "NaN"),
    ] {
        let json = serde_json::to_string(&Value::make_number(num)).unwrap();
        assert_eq!(json, format!(r#"{{"_kind":"number","val":"{val}"}}"#));

        let value = Value::make_number_unit(num, get_unit_or_default("m/s"));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(
            json,
            format!(r#"{{"_kind":"number","val":"{val}","unit":"m/s"}}"#)
        );
    }
}

#[test]
fn test_json_decode_non_finite_num() {
    let mut value: Value = serde_json::from_str(r#"{"_kind":"number","val":"INF"}"#).unwrap();
    assert_eq!(value, Value::make_number(f64::INFINITY));

    value = serde_json::from_str(r#"{"_kind":"number","val":"-INF"}"#).unwrap();
    assert_eq!(value, Value::make_number(f64::NEG_INFINITY));

    value = serde_json::from_str(r#"{"_kind":"number","val":"NaN"}"#).unwrap();
    assert!(Number::try_from(&value).unwrap().value.is_nan());

    value = serde_json::from_str(r#"{"_kind":"number","val":"INF","unit":"m/s"}"#).unwrap();
    assert_eq!(
        value,
        Value::make_number_unit(f64::INFINITY, get_unit_or_default("m/s"))
    );

    value = serde_json::from_str(r#"{"_kind":"number","val":"NaN","unit":"m/s"}"#).unwrap();
    let num = Number::try_from(&value).unwrap();
    assert!(num.value.is_nan());
    assert_eq!(num.unit, Some(get_unit_or_default("m/s")));

    // A plain string stays a string
    value = serde_json::from_str(r#""INF""#).unwrap();
    assert_eq!(value, Value::make_str("INF"));
}

#[test]
fn test_json_decode_invalid_number_string() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"number","val":"foo"}"#).is_err());
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"number","val":"inf"}"#).is_err());
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
