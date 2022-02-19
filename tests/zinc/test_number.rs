// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Number

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::units::get_unit_or_default;
use libhaystack::val::*;

#[test]
fn test_zinc_serialize_num() {
    let mut value = Value::Number(Number {
        value: 100.0,
        unit: None,
    });

    let mut zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "100");

    value = Value::make_number_unit(42.0, get_unit_or_default("m/s"));

    zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "42m/s");
}

#[test]
fn test_zinc_decode_num_euro() {
    let value: Value = from_str("100EUR").unwrap();
    assert_eq!(
        value,
        Value::make_number_unit(100.0, get_unit_or_default("EUR"))
    );
}

#[test]
fn test_zinc_decode_num_exp() {
    let value: Value = from_str("1E1").unwrap();
    assert_eq!(value, Value::make_number(10.0));

    let value: Value = from_str("1E+2").unwrap();
    assert_eq!(value, Value::make_number(100.0));

    let value: Value = from_str("1E-1").unwrap();
    assert_eq!(value, Value::make_number(0.1));
}

#[test]
fn test_zinc_de_serialize_number() {
    let mut value: Value = from_str("22.44").unwrap();
    assert_eq!(value, Value::make_number(22.44));

    value = from_str("-3.0").unwrap();
    assert_eq!(value, Value::make_number(-3.0));

    value = from_str("-INF").unwrap();
    assert_eq!(value, Value::make_number(f64::NEG_INFINITY));
    value = from_str("INF").unwrap();
    assert_eq!(value, Value::make_number(f64::INFINITY));

    value = from_str("NaN").unwrap();
    assert!(f64::try_from(&value).unwrap().is_nan());

    value = from_str("91_45").unwrap();
    assert_eq!(value, Value::make_number(9145.0));

    value = from_str("42s").unwrap();
    assert_eq!(
        value,
        Value::make_number_unit(42.0, get_unit_or_default("s"))
    );

    value = from_str("7.5026717880998035°F").unwrap();
    assert_eq!(
        value,
        Value::make_number_unit(7.5026717880998035, get_unit_or_default("°F"))
    );
}
