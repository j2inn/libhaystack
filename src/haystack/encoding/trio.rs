// Copyright (C) 2020 - 2024, J2 Innovations

//! Haystack Trio Encoding
//!
//! Trio ("Text Record Input/Output") is a human-friendly plain-text format
//! derived from YAML that is used for hand-authoring Haystack data.
//!
//! See <https://project-haystack.org/doc/docHaystack/Trio>

#[cfg(feature = "trio-decoding")]
pub mod decode;
#[cfg(feature = "trio-encoding")]
pub mod encode;
