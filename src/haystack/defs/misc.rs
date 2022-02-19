// Copyright (C) 2020 - 2022, J2 Innovations

//! Misc functions and utils

use crate::{
    encoding::zinc::decode::from_str,
    val::{Dict, Str, Value},
};

/// Parse the multi-line values into a list of dict.
///
/// # Arguments
/// - param val The value string to parse.
/// # Returns
/// A list of dicts.
///
pub(super) fn parse_multi_line_string_to_dicts(val: &Str) -> Vec<Dict> {
    val.value
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| match from_str(&format!("{{{line}}}")) {
            Ok(Value::Dict(dict)) => dict,
            Ok(_) | Err(_) => Dict::default(),
        })
        .filter(|dict| !dict.is_empty())
        .collect()
}
