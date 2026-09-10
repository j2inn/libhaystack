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
    assert_eq!(uri.value(), "/foo/baz/bar.txt".to_string());
}

#[test]
fn test_uri_from_string() {
    let uri = Uri::from("/foo/baz/bar.txt".to_string());
    assert_eq!(uri.value(), "/foo/baz/bar.txt".to_string());
}

#[test]
fn test_uri_from_box_str() {
    let boxed: Box<str> = "/foo/baz/bar.txt".into();
    let uri = Uri::from(boxed);
    assert_eq!(uri.value(), "/foo/baz/bar.txt");
}

#[test]
fn test_uri_into_inner() {
    let uri = Uri::make("/a/b");
    let inner: Box<str> = uri.into_inner();
    assert_eq!(&*inner, "/a/b");
}
