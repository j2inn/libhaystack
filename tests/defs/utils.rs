// Copyright (C) 2020 - 2022, J2 Innovations

//! Defs utils

use libhaystack::haystack::encoding::zinc::decode::*;
use libhaystack::haystack::val::{Grid, Value};
use std::fs;

// Parse the def file
pub(crate) fn parse_def() -> Grid {
    parse_def_file("defs.zinc")
}

// Parse the test features def file
pub(crate) fn parse_features_def() -> Grid {
    parse_def_file("defsWithFeatures.zinc")
}

// Parse a def file
pub(crate) fn parse_def_file(file: &str) -> Grid {
    let file = format!(
        "{root}/tests/defs/{file}",
        root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(format!(
            "{cwd}",
            cwd = std::env::current_dir().unwrap().display().to_string()
        ))
    );

    let string = fs::read_to_string(file).expect("Invalid zinc test file");
    let value: Value = from_str(&string).expect("Value");

    Grid::try_from(&value).expect("Grid")
}
