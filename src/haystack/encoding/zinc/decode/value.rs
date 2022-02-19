// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Value decoding
use super::parser::Parser;
use crate::haystack::val::Value;
use std::io::{Cursor, Error};

/// Utility function for parsing haystack [Value](crate::val::value::Value) from a `&str`
///
/// # Example
/// ```
/// use libhaystack::val::uri::*;
/// use libhaystack::encoding::zinc::decode::*;
///
/// let val = from_str("`/a/sample/uri`").expect("A Value");
/// assert!(&val.is_uri());
///
/// assert_eq!(Uri::try_from(&val).unwrap().value, "/a/sample/uri");
/// ```
pub fn from_str(str: &str) -> Result<Value, Error> {
    let mut input = Cursor::new(str.as_bytes());
    let mut parser = Parser::make(&mut input)?;
    parser.parse_value()
}
