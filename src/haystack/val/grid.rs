// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Grid

use crate::dict;
use crate::haystack::val::dict::*;
use crate::haystack::val::Value;
use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::Index;

/// Grid Column
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Debug, Default)]
pub struct Column {
    /// Column name
    pub name: String,
    /// Optional column meta-data dictionary
    pub meta: Option<Dict>,
}

/// The version of the grid format supported by this library
pub const GRID_FORMAT_VERSION: &str = "3.0";

/// Haystack Grid
///
/// # Example
/// Create [Grid](crate::val::Grid) from list of [Dict](crate::val::Dict)s
/// ```
/// use libhaystack::dict;
/// use libhaystack::val::*;
///
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
///   let grid = Grid::make_from_dicts(rows);
///   assert_eq!(grid.is_empty(), false);
///   // Get the first row from `Grid`
///   assert_eq!(grid[0], dict! {
///    "site" => Value::make_marker(),
///    "dis" => Value::make_str("Site")
///   });
///```
///
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Grid {
    /// Optional `Grid` meta-data dictionary
    pub meta: Option<Dict>,
    /// List of the columns this `Grid` has
    pub columns: Vec<Column>,
    /// List of the row for this `Grid`
    pub rows: Vec<Dict>,
    /// The version of this grid
    pub ver: String,
}

impl Grid {
    /// Create an empty `Grid`
    pub fn make_empty() -> Self {
        Grid {
            meta: None,
            columns: vec![Column {
                name: String::from("empty"),
                meta: None,
            }],
            rows: Vec::default(),
            ver: GRID_FORMAT_VERSION.to_string(),
        }
    }

    /// Constructs a Grid from a list of `Dict`s
    pub fn make_from_dicts(rows: Vec<Dict>) -> Self {
        let mut col_names = HashSet::<String>::new();

        rows.iter().for_each(|el| {
            el.keys().for_each(|col_name| {
                col_names.insert(col_name.clone());
            })
        });

        let mut columns: Vec<Column> = col_names
            .iter()
            .map(|col_name| Column {
                name: col_name.clone(),
                meta: None,
            })
            .collect();
        columns.sort_by(|a, b| a.name.cmp(&b.name));

        Grid {
            meta: None,
            columns,
            rows,
            ver: GRID_FORMAT_VERSION.to_string(),
        }
    }

    /// Constructs a Grid from a list of `Dict`s with a meta Dict
    /// # Example
    /// Create [Grid](crate::val::Grid) from list of [Dict](crate::val::Dict)s with metadata
    /// ```
    /// use libhaystack::dict;
    /// use libhaystack::val::*;
    ///
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
    ///   let grid = Grid::make_from_dicts_with_meta(rows, dict! {
    ///    "dis" => Value::make_str("A Test Grid")
    ///   });
    ///   assert_eq!(grid.is_empty(), false);
    ///   assert_eq!(grid.meta, Some(dict! {
    ///    "dis" => Value::make_str("A Test Grid")
    ///   }));
    ///```
    pub fn make_from_dicts_with_meta(rows: Vec<Dict>, meta: Dict) -> Self {
        let mut grid = Grid::make_from_dicts(rows);
        grid.meta = Some(meta);
        grid
    }

    /// Create an Err `Grid`
    pub fn make_err(dis: &str) -> Self {
        Grid {
            meta: Some(dict! {"err" => Value::Marker, "dis" => dis.into()}),
            columns: vec![Column {
                name: String::from("empty"),
                meta: None,
            }],
            rows: Vec::default(),
            ver: GRID_FORMAT_VERSION.to_string(),
        }
    }

    /// True if `Grid` has no rows
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns number of rows in this `Grid`
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True if `Grid` is an error grid
    pub fn is_err(&self) -> bool {
        if let Some(meta) = &self.meta {
            meta.has_marker("err")
        } else {
            false
        }
    }
}

/// Implements the `Default` trait for the `Grid`
impl Default for Grid {
    fn default() -> Self {
        Grid {
            columns: Vec::default(),
            meta: None,
            rows: Vec::default(),
            ver: GRID_FORMAT_VERSION.to_string(),
        }
    }
}

/// Implements the `Index` trait for the `Grid`
/// this allows usage such `mygrid[1]`
impl Index<usize> for Grid {
    type Output = Dict;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

/// Grid iterator helper
pub struct IterHelper<'a> {
    iter: ::std::slice::Iter<'a, Dict>,
}

/// Implement `Iterator` trait for the helper
impl<'a> Iterator for IterHelper<'a> {
    type Item = &'a Dict;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Implement `IntoIterator` for `Grid`
impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Dict;
    type IntoIter = IterHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper {
            iter: self.rows.iter(),
        }
    }
}

/// Converts from `Grid` to a `Grid` `Value`
impl From<Grid> for Value {
    fn from(value: Grid) -> Self {
        Value::Grid(value)
    }
}

/// Tries to convert from `Value` to a `Grid`
impl TryFrom<&Value> for Grid {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Grid(v) => Ok(v.clone()),
            _ => Err("Value is not an `Grid`"),
        }
    }
}
