// Copyright (C) 2020 - 2022, J2 Innovations

//! Tests the Hayson file decoding

#[cfg(test)]
extern crate libhaystack;

use libhaystack::val::*;

use std::fs;

#[test]
fn test_json_file_parse() {
    let string = fs::read_to_string("benches/json/points.json").expect("Invalid json test file");

    let value: Value = serde_json::from_str(&string).unwrap();

    let grid = Grid::try_from(&value).unwrap();

    assert!(!grid.is_empty());
}
