// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Filtered Grid

use super::super::Filter;
use super::Filtered;
use super::ListFiltered;
use crate::val::Dict;
use crate::val::Grid;

/// Implement the Filtered trait for Grid.
impl<'a> Filtered<'a, Option<&'a Dict>> for Grid {
    /// This will find the first matching Dict in the grid, one None
    //
    /// # Example
    /// ```
    ///  use libhaystack::dict;
    ///  use libhaystack::val::{Dict, Grid, Value};
    ///  use libhaystack::filter::{Filter, Filtered};
    ///  let rows = vec![
    ///   dict! {
    ///    "site" => Value::make_marker(),
    ///    "dis" => Value::make_str("Site")
    ///   },
    ///   dict! {
    ///    "equip" => Value::make_marker(),
    ///    "navName" => Value::make_str("Equip")
    ///   },
    ///   ];
    ///  let grid = Grid::make_from_dicts(rows);
    ///  assert_eq!(grid.filter(&Filter::try_from("equip and navName").expect("Filter")), Some(&dict! {
    ///    "equip" => Value::make_marker(),
    ///    "navName" => Value::make_str("Equip")
    ///   }));
    /// ```
    fn filter(&'a self, filter: &Filter) -> Option<&'a Dict> {
        for dict in &self.rows {
            if dict.filter(filter) {
                return Some(dict);
            }
        }
        None
    }
}

impl<'a> ListFiltered<'a, Dict> for Grid {
    /// Find all rows in the grid that match the given filter.
    ///
    /// # Example
    /// ```
    ///  use libhaystack::dict;
    ///  use libhaystack::val::{Dict, Grid, Value};
    ///  use libhaystack::filter::{Filter, ListFiltered};
    ///  let rows = vec![
    ///   dict! { "a" => Value::make_marker()},
    ///   dict! { "b" => Value::make_marker()}];
    ///  let grid = Grid::make_from_dicts(rows);
    ///  let res = grid.filter_all(&Filter::try_from("a or b").unwrap());
    ///  assert_eq!(res.len(), 2);
    /// ```
    fn filter_all(&'a self, filter: &Filter) -> Vec<&'a Dict> {
        let mut dicts = Vec::<&'a Dict>::new();
        for dict in &self.rows {
            if dict.filter(filter) {
                dicts.push(dict)
            }
        }
        dicts
    }
}
