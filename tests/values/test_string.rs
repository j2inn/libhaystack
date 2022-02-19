// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Str

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_str_make_value() {
    let str = Str::make("foo");

    let value: Value = str.into();

    assert!(value.is_str());
    assert!(!value.is_number());

    assert_eq!(Str::try_from(&value), Ok(Str::make("foo")));
}

#[test]
fn test_ref_from() {
    let str = Str::from("id");
    assert_eq!(str.value, "id".to_string());
}
