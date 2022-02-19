// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! Haystack Tag Value Types
//!
//! Haystack defines a type system the uses both scalar and collection types.
//! See [Haystack kinds](https://project-haystack.org/doc/docHaystack/Kinds)
//!
//! # Scalars
//! The haystack specification defines scalar such as `Number`, `Bool`, `Str`, ...
//!
//! ## Examples
//! Creating a scalar Haystack number value
//! ```
//! use libhaystack::val::number::*;
//! use libhaystack::units::get_unit_or_default;
//! use libhaystack::encoding::zinc::encode::*;
//!
//! let num = Number::make_with_unit(42.0, get_unit_or_default("°F"));
//! assert_eq!(num.value, 42.0);
//!
//! // Numbers can be encoded, to Zinc for example.
//! assert_eq!(num.to_zinc_string(), Ok("42°F".to_string()));
//!
//! ```
//! # Collections
//! The haystack specification defines collection types as `List`, `Dict`, `Grid`
//!
//! ## Examples
//! Creating a List
//! ```
//! use libhaystack::val::*;
//! use libhaystack::encoding::json::encode::*;
//!
//! let values = vec![Value::make_true(), Value::make_number(23.0)];
//! assert_eq!(values.len(), 2);
//!
//! // Test the list content using the JSON encoding
//! assert_eq!(serde_json::to_string(&values).unwrap(), "[true,23]");
//! ```
//! Creating a Dict
//! ```
//! use libhaystack::dict;
//! use libhaystack::val::*;
//! use libhaystack::encoding::json::encode::*;
//!
//! let dict = dict!{"number" => Value::make_number(42.0), "string" => Value::make_str("the answer")};
//! assert_eq!(dict.len(), 2);
//!
//! // Test the dict content using the JSON encoding
//! assert_eq!(serde_json::to_string(&dict).unwrap(), r#"{"number":42,"string":"the answer"}"#);
//! ```
//!
//! Creating a Grid
//! ```
//! use libhaystack::dict;
//! use libhaystack::val::*;
//! use libhaystack::encoding::zinc::encode::*;
//!
//! let grid = Value::make_grid_from_dicts([
//!    dict!{"id" => Value::make_ref("id1"), "value" => Value::make_uri("/an/uri")},
//!    dict!{"id" => Value::make_ref("id2"), "value" => Value::make_marker()},
//! ].to_vec());
//! assert!(grid.is_grid());
//!
//! // Test the grid content using the Zinc encoding
//! assert_eq!(grid.to_zinc_string(), Ok("ver:\"3.0\"\nid,value\n@id1,`/an/uri`\n@id2,M\n\n".into()));
//! ```
//!

pub mod boolean;
pub mod coord;
pub mod date;
pub mod datetime;
pub mod dict;
pub mod grid;
pub mod kind;
pub mod list;
pub mod marker;
pub mod na;
pub mod number;
pub mod reference;
pub mod remove;
pub mod string;
pub mod symbol;
pub mod time;
pub mod uri;
pub mod value;
pub mod xstr;

pub use crate::haystack::val::boolean::*;
pub use crate::haystack::val::coord::*;
pub use crate::haystack::val::date::*;
pub use crate::haystack::val::datetime::*;
pub use crate::haystack::val::dict::*;
pub use crate::haystack::val::grid::*;
pub use crate::haystack::val::list::*;
pub use crate::haystack::val::marker::*;
pub use crate::haystack::val::na::*;
pub use crate::haystack::val::number::*;
pub use crate::haystack::val::reference::*;
pub use crate::haystack::val::remove::*;
pub use crate::haystack::val::string::*;
pub use crate::haystack::val::symbol::*;
pub use crate::haystack::val::time::*;
pub use crate::haystack::val::uri::*;
pub use crate::haystack::val::value::*;
pub use crate::haystack::val::xstr::*;
