// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! Haystack encodings
//!
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "zinc")]
pub mod zinc;

pub mod decode_ref_dis_factory;
