// Copyright (C) 2020 - 2022, J2 Innovations

//! Test List

#[cfg(test)]
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_list_encode() {
    let value = Value::make_list(vec![
        Value::make_marker(),
        Value::make_bool(true),
        Value::make_str("foo"),
        Value::make_list(vec![Value::make_number(42.0)]),
    ]);

    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), r#"[M,T,"foo",[42]]"#);
}
#[test]
fn test_zinc_list_decode() {
    let value: Value = from_str(r#"[M, T,"foo" , [  42.0] ]"#).unwrap();
    let list = Value::make_list(vec![
        Value::make_marker(),
        Value::make_bool(true),
        Value::make_str("foo"),
        Value::make_list(vec![Value::make_number(42.0)]),
    ]);
    assert_eq!(value, list);
}
