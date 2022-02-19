// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Symbol

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_symbol_make_value() {
    let sym = Symbol::make("symbol");

    let value: Value = sym.into();

    assert!(value.is_symbol());
    assert!(!value.is_str());

    assert_eq!(Symbol::try_from(&value), Ok(Symbol::make("symbol")));
}

#[test]
fn test_symbol_from() {
    let sym = Symbol::from("some-sym");
    assert_eq!(sym.value, "some-sym".to_string());
}
