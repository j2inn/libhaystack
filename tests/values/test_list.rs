// Copyright (C) 2020 - 2022, J2 Innovations

//! Test List

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_list_make_value() {
    let list: List = vec!["a".into(), 100.into(), true.into()];

    let value: Value = list.into();

    assert!(value.is_list());
    assert!(!value.is_bool());

    assert_eq!(
        List::try_from(&value),
        Ok(vec!["a".into(), 100.into(), true.into()])
    );
}
