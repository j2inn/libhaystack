// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Uri

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_uri_make_value() {
    let uri = Uri::make("/a/b");

    let value: Value = uri.into();

    assert!(value.is_uri());
    assert!(!value.is_ref());

    assert_eq!(Uri::try_from(&value), Ok(Uri::make("/a/b")));
}

#[test]
fn test_uri_from() {
    let uri = Uri::from("/foo/baz/bar.txt");
    assert_eq!(uri.value, "/foo/baz/bar.txt".to_string());
}
