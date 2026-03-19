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

#[test]
fn test_ref_from_string() {
    let str = Str::from("id".to_string());
    assert_eq!(str.value, "id".to_string());
}

#[test]
fn test_str_as_str() {
    let str = Str::make("foo");
    assert_eq!(str.as_str(), "foo");
}

#[test]
fn test_str_display() {
    let str = Str::make("foo");
    assert_eq!(format!("{}", str), "foo");
}

#[test]
fn test_str_deref() {
    let str = Str::make("hello world");
    // &Str coerces to &str; all str methods are available
    assert_eq!(str.len(), 11);
    assert!(str.contains("world"));
    assert!(str.starts_with("hello"));
}

#[test]
fn test_str_as_ref() {
    fn takes_as_ref(s: impl AsRef<str>) -> usize {
        s.as_ref().len()
    }
    assert_eq!(takes_as_ref(Str::make("foo")), 3);
}

#[test]
fn test_str_into_string() {
    let str = Str::make("foo");
    let owned: String = str.into();
    assert_eq!(owned, "foo");
}

#[test]
fn test_string_into_value() {
    let value = Value::from(String::from("foo"));
    assert!(value.is_str());
    assert_eq!(Str::try_from(&value), Ok(Str::make("foo")));
}

#[test]
fn test_str_partial_eq_str_slice() {
    let str = Str::make("foo");
    assert_eq!(str, *"foo");
    assert_ne!(str, *"bar");
}

#[test]
fn test_str_partial_eq_string() {
    let str = Str::make("foo");
    assert_eq!(str, String::from("foo"));
    assert_ne!(str, String::from("bar"));
}
