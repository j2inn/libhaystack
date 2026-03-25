// Copyright (C) 2020 - 2022, J2 Innovations

//! Brio binary encoding/decoding for Haystack values.
//!
//! Brio is a compact binary format used by Project Haystack and the Haxall platform.
//! It provides efficient binary serialization of all Haystack value types.
//!
//! # Features
//! - `brio-encoding`: enables the [`encode`] module
//! - `brio-decoding`: enables the [`decode`] module
//!
//! # Example – round-trip a `Dict`
//! ```rust
//! use libhaystack::val::*;
//! use libhaystack::dict;
//! use libhaystack::encoding::brio::encode::ToBrio;
//! use libhaystack::encoding::brio::decode::from_brio;
//!
//! let val = Value::make_dict(dict! {
//!     "site" => Value::make_marker(),
//!     "dis"  => Value::from("Main Campus")
//! });
//!
//! let bytes = val.to_brio_vec().expect("encode");
//! let decoded = from_brio(&mut bytes.as_slice()).expect("decode");
//! assert_eq!(val, decoded);
//! ```

#[cfg(any(feature = "brio-encoding", feature = "brio-decoding"))]
pub mod consts;

#[cfg(feature = "brio-encoding")]
pub mod encode;

#[cfg(feature = "brio-decoding")]
pub mod decode;

#[cfg(all(test, feature = "brio-encoding", feature = "brio-decoding"))]
mod haxall_fixtures;

#[cfg(all(test, feature = "brio-encoding", feature = "brio-decoding"))]
mod json_fixture_tests;
