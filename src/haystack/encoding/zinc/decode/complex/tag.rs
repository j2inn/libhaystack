// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Haystack tag decoding

use crate::haystack::val::Value;

use super::id::*;
use super::value::*;

// Zinc tag pair
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct TagPair {
    pub(super) name: String,
    pub(super) value: Value,
}

// Parse a Haystack tag
