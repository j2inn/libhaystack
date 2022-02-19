// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Str

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_str_encode() {
    let value = Value::make_str("Foo");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#""Foo""#);
}

#[test]
fn test_zinc_serialize_str_ecapse() {
    let value = Value::make_str("\n\t$");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#""\n\t\$""#);

    let value = Value::make_str("unicode char ש");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#""unicode char ש""#);

    let value = Value::make_str("_ \\ \" \n \r \t \u{0011} _");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#""_ \\ \" \n \r \t \u0011 _""#);
}
#[test]
fn test_zinc_str_decode() {
    let mut value: Value = from_str(r#""a string value""#).unwrap();
    assert_eq!(value, Value::make_str("a string value"));

    value = from_str("\"\\t\"").unwrap();
    assert_eq!(value, Value::make_str("\t"));

    assert!(from_str(r#""non teminated"#).is_err());
}

#[test]
fn test_zinc_str_decode_bad() {
    assert!(from_str(r#""non terminated"#).is_err());
}
