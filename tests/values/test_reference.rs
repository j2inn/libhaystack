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

    let id = Ref::generate();
    assert!(id.value.len() > 13);
}

#[test]
fn test_ref_make_with_dis() {
    let id = Ref::make("id", Some("dis"));
    assert_eq!(id.value, "id".to_string());
    assert_eq!(id.dis, Some("dis".to_string()));
}

#[test]
fn test_ref_make_with_dis_none() {
    let id = Ref::make("id", None);
    assert_eq!(id.value, "id".to_string());
    assert_eq!(id.dis, None);
}

#[test]
fn test_ref_from_string() {
    let id = Ref::from("id".to_string());
    assert_eq!(id.value, "id".to_string());
}

#[test]
fn test_ref_as_str() {
    let id = Ref::from("id");
    assert_eq!(id.as_str(), "id");
}

#[test]
fn test_ref_cmp() {
    assert!(Ref::from("abc") < Ref::from("xyz"));
}
