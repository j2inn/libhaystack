// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Uri

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_uri_encode() {
    let value = Value::make_uri("http://zoo.bar");
    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "`http://zoo.bar`");
}
#[test]
fn test_zinc_uri_decode() {
    let mut value: Value = from_str("`http://zoo.bar`").unwrap();
    assert_eq!(value, Value::make_uri("http://zoo.bar"));
    value = from_str("`http://zoo.\\`bar`").unwrap();
    assert_eq!(value, Value::make_uri("http://zoo.`bar"));
}

#[test]
fn test_zinc_uri_decode_bad() {
    assert!(from_str(r#"`^"#).is_err());
}

#[test]
fn test_zinc_uri_unicode() {
    let value = Value::make_uri("/fῶo");
    let zinc = to_zinc_string(&value).unwrap();
    assert_eq!(zinc, r#"`/f\u1ff6o`"#);
    assert_eq!(from_str(&zinc).unwrap(), value);

    let value = Value::make_uri("a😀b");
    let zinc = to_zinc_string(&value).unwrap();
    assert_eq!(zinc, r#"`a\ud83d\ude00b`"#);
    assert_eq!(from_str(&zinc).unwrap(), value);

    assert_eq!(
        from_str(r#"`a\u{1F600}b`"#).unwrap(),
        Value::make_uri("a😀b")
    );
}
