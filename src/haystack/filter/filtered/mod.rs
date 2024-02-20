// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Filtered implementations

use super::Filter;

pub mod dict;
pub mod grid;

/// Generic trait that enables filtering evaluation over an object.
/// Specifically this would be implemented by [Dict](crate::val::Dict)
/// and [Grid](crate::val::Grid).
///
// # Example
/// ```
///  use libhaystack::dict;
///  use libhaystack::val::{Dict, Value};
///  use libhaystack::filter::{Filter, Filtered};
///
///  let dict = dict!{
///     "site" => Value::make_marker(),
///     "dis" => Value::make_str("Some site")
///  };
///  assert!(dict.filter(&Filter::try_from("site and dis").expect("Filter")));
/// ```
pub trait Filtered<'a, T> {
    /// Apply the `filter` against self.
    fn filter(&'a self, filter: &Filter) -> T;
}

/// Generic trait that enables filtering evaluation over a list of objects
/// that implement the `Filtered` trait.
/// Specifically this would be implemented by [Grid](crate::val::Grid).
///
/// # Example
/// ```
///  use libhaystack::dict;
///  use libhaystack::val::{Dict, Grid, Value};
///  use libhaystack::filter::{Filter, ListFiltered};
///
///  let rows = vec![
///   dict! { "a" => Value::make_marker()},
///   dict! { "b" => Value::make_marker()}];
///  let grid = Grid::make_from_dicts(rows);
///  let res = grid.filter_all(&Filter::try_from("a or b").unwrap());
///  assert_eq!(res.len(), 2);
/// ```
pub trait ListFiltered<'a, T: 'a> {
    /// Apply the `filter` against self.
    fn filter_all(&'a self, filter: &Filter) -> Vec<&'a T>;
}
