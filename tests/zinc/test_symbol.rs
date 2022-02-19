// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Symbol

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_symbol_encode() {
    let value: Value = Value::make_symbol("test");
    assert_eq!(value, Value::make_symbol("test"));

    assert_eq!(value.to_zinc_string(), Ok("^test".into()));
}

#[test]
fn test_zinc_symbol_decode() {
    let value: Value = from_str("^test").unwrap();
    assert_eq!(value, Value::make_symbol("test"));
}

#[test]
fn test_zinc_symbol_decode_bad() {
    assert!(from_str(r#"^^"#).is_err());
}
