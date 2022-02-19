// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Bool

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_bool_from_value() {
    let value: Value = false.into();

    assert!(value.is_bool());

    assert_eq!(Bool::try_from(&value), Ok(false.into()));
    assert_eq!(bool::try_from(&value), Ok(false));

    let value: Bool = true.into();

    assert!(bool::from(value));
}
