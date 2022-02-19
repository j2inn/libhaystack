// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Ref

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_ref_make_value() {
    let id = Ref::make("foo", Some("bar"));

    let value: Value = id.into();

    assert!(value.is_ref());
    assert!(!value.is_number());

    assert_eq!(Ref::try_from(&value), Ok(Ref::make("foo", Some("bar"))));
}

#[test]
fn test_ref_make() {
    let id = Ref::from("id");
    assert_eq!(id.value, "id".to_string());

    let id = Ref::gen();
    assert!(id.value.len() > 13);
}

#[test]
fn test_ref_cmp() {
    assert!(Ref::from("abc") < Ref::from("xyz"));
}
