// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Bool Zinc
//!
#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_bool_encode() {
    let value = Bool::from(true);
    let zinc = value.to_zinc_string();
    assert_eq!(zinc, Ok("T".into()));
    assert_eq!(Value::from(value).to_zinc_string(), Ok("T".into()));

    let value = Bool::from(false);
    let zinc = value.to_zinc_string();
    assert_eq!(zinc, Ok("F".into()));
}

#[test]
fn test_zinc_bool_decode() {
    let value = from_str("T");
    assert_eq!(value.ok(), Some(true.into()));

    let value = from_str("F");
    assert_eq!(value.ok(), Some(false.into()));

    assert!(from_str("X").is_err());
}
