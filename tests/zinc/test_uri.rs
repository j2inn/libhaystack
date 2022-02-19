// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Uri

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_uri_encode() {
    let value = Value::make_uri("http://zoo.bar");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "`http://zoo.bar`");
}
#[test]
fn test_zinc_uri_decode() {
    let mut value: Value = from_str("`http://zoo.bar`").unwrap();
    assert_eq!(value, Value::make_uri("http://zoo.bar"));
    value = from_str("`http://zoo.\\`bar`").unwrap();
    assert_eq!(value, Value::make_uri("http://zoo.`bar"));
}

#[test]
fn test_zinc_uri_decode_bad() {
    assert!(from_str(r#"`^"#).is_err());
}
