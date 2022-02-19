// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Ref

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_ref_encode() {
    let mut value = Value::Ref(Ref {
        value: String::from("someId"),
        dis: Some(String::from("dis")),
    });

    let mut zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#"@someId "dis""#);

    value = Value::make_ref("id");
    zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), "@id");
}

#[test]
fn test_zinc_ref_decode() {
    let mut value: Value = from_str("@test").unwrap();
    assert_eq!(value, Value::make_ref("test"));

    value = from_str("@5f632023-82b-08606e").unwrap();
    assert_eq!(value, Value::make_ref("5f632023-82b-08606e"));

    value = from_str(r#"@foo "some dis""#).unwrap();
    assert_eq!(value, Value::make_ref_with_dis("foo", "some dis"));
}

#[test]
fn test_zinc_ref_bad() {
    assert!(from_str(r#"@"#).is_err());
}
