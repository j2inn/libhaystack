// Copyright (C) 2020 - 2022, J2 Innovations

//! Test XStr

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_xstr_make_value() {
    let xstr = XStr::make("type", "value");

    let value: Value = xstr.into();

    assert!(value.is_xstr());
    assert!(!value.is_symbol());

    assert_eq!(XStr::try_from(&value), Ok(XStr::make("type", "value")));
}

#[test]
fn test_xstr_into_inner() {
    let xstr = XStr::make("type", "value");
    let (r#type, value) = xstr.into_inner();
    assert_eq!(&*r#type, "type");
    assert_eq!(&*value, "value");
}
