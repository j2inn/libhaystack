// Copyright (C) 2020 - 2022, J2 Innovations

//! Test XStr

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_xstr_encode() {
    let value = Value::make_xstr_from("foo", "bar");

    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#"Foo("bar")"#);
}
#[test]
fn test_zinc_xstr_decode() {
    let value: Value = from_str(r#"Foo("bar")"#).unwrap();
    assert_eq!(value, Value::make_xstr_from("Foo", "bar"));
}
