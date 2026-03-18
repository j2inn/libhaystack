// Copyright (C) 2020 - 2022, J2 Innovations

//! Implement Haystack [JSON (Hayson) Encoding](https://project-haystack.org/doc/docHaystack/Json)
//!
//! The implementation is based on the excellent [serde_json](https://docs.serde.rs/serde_json/) de/serialization framework.
//!
//! # Examples
//!
//! Encoding
//! ```
//! use libhaystack::val::*;
//! let json = serde_json::to_string(&Value::make_true());
//!
//! assert_eq!(json.unwrap(), "true");
//!
//! let json = serde_json::to_string(&Value::make_coord_from(29.9792, 31.1342));
//!
//! assert_eq!(json.unwrap(), r#"{"_kind":"coord","lat":29.9792,"lng":31.1342}"#);
//!
//! ```
//!
//! Decoding
//! ```
//! use libhaystack::val::*;
//! let list: Vec<Value> = serde_json::from_str(r#"[100, "foo", {"_kind": "marker"}]"#).expect("Valid JSON");
//!
//! assert_eq!(list.len(), 3);
//!
//! assert_eq!(list, vec![Value::make_number(100.0), Value::make_str("foo"), Value::make_marker()]);
//!
//! ```

/// Hayson decoder
#[cfg(feature = "json-decoding")]
pub mod decode;
#[cfg(feature = "json-encoding")]
/// Hayson encoder
pub mod encode;
