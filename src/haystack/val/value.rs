// Copyright (C) 2020 - 2022, J2 Innovations

use crate::haystack::val::boolean::*;
use crate::haystack::val::coord::*;
use crate::haystack::val::date::*;
use crate::haystack::val::datetime::*;
use crate::haystack::val::dict::*;
use crate::haystack::val::grid::*;
use crate::haystack::val::list::*;
use crate::haystack::val::number::*;
use crate::haystack::val::reference::*;
use crate::haystack::val::string::*;
use crate::haystack::val::symbol::*;
use crate::haystack::val::time::*;
use crate::haystack::val::uri::*;
use crate::haystack::val::xstr::*;
use crate::units::Unit;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

///
/// The `Value` object is the universal type that can have any of supported
/// tag values specified by the [Project Haystack 4.0 spec](https://project-haystack.org/doc/docHaystack/Kinds).
///
/// It is implemented as an Algebraic type (enum) than can be constructed
/// from build-in types, such as `bool` or `i32`, and dedicated Haystack types such as `Str`
///
/// # Usage
///
/// Creating a scalar Haystack number value
/// ```
/// use libhaystack::val::*;
/// use libhaystack::units::get_unit_or_default;
///
/// let num = Value::make_number_unit(42.0, get_unit_or_default("°F"));
///
/// assert!(num.is_number());
/// assert_eq!(Number::try_from(&num).unwrap().unit, Some(get_unit_or_default("°F")));
/// ```
///
/// Creating complex structures such as a Dict
/// ```
/// use libhaystack::dict;
/// use libhaystack::val::*;
///
/// let dict = Value::make_dict(dict!("strTagName" => Value::from("Str Tag value"),
///                  "marker" => Value::make_marker()));
/// assert!(dict.is_dict());
/// ```
///
/// Creating a Haystack Grid
/// ```
/// use libhaystack::dict;
/// use libhaystack::val::*;
///
/// let grid = Value::make_grid_from_dicts(vec![
///     dict!("dis" => Value::from("First dict row"),
///           "id" => Value::make_ref_gen()),
///     dict!("dis" => Value::from("Second dict row"),
///           "id" => Value::make_ref_gen())
/// ]);
/// assert!(grid.is_grid());
/// assert_eq!(Grid::try_from(&grid).unwrap().len(), 2);
/// ```
///
#[derive(PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Value {
    /// No value
    Null,
    /// A remove tag
    Remove,
    /// Marker tag
    Marker,
    /// Bool true/false
    Bool(Bool),
    /// Na tag
    Na,
    /// Number floating point value and optional unit
    Number(Number),
    // String value
    Str(Str),
    /// URI with string value
    Uri(Uri),
    /// Ref with string value
    Ref(Ref),
    /// A symbol with string value
    Symbol(Symbol),
    /// Date year, month, date
    Date(Date),
    /// Time hour, minutes, seconds with optional millis
    Time(Time),
    /// DateTime date, time and timezone
    DateTime(DateTime),
    /// Coordinate latitude and longitude
    Coord(Coord),
    /// XStr with type and value
    XStr(XStr),
    /// List of tags
    List(List),
    /// Dictionary of `String` key and `Value` values
    Dict(Dict),
    /// Haystack Grid
    Grid(Grid),
}

///
/// Implements the utility functions for creating and getting the current type
///
impl Value {
    /// True if this `Value` is a haystack `Null`
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// True if this Haystack `Value` is not null
    pub fn has_value(&self) -> bool {
        !self.is_null()
    }

    /// Construct a `Remove` `Value`
    pub fn make_remove() -> Value {
        Value::Remove
    }

    /// True if this `Value` is a haystack `Remove`
    pub fn is_remove(&self) -> bool {
        matches!(self, Value::Remove)
    }

    /// Construct a `Marker` `Value`
    pub fn make_marker() -> Value {
        Value::Marker
    }

    /// True if this `Value` is a haystack `Marker`
    pub fn is_marker(&self) -> bool {
        matches!(self, Value::Marker)
    }

    /// Construct a `Bool` `Value`
    pub fn make_bool(value: bool) -> Value {
        Value::Bool(Bool::from(value))
    }

    /// Construct a `Bool` 'true' `Value`
    pub fn make_true() -> Value {
        Value::Bool(true.into())
    }

    /// True if [Value](crate::val::Value) is a `true` (Bool)[crate::val::Bool]
    pub fn is_true(&self) -> bool {
        match self {
            Value::Bool(v) => v.value,
            _ => false,
        }
    }

    /// True if [Value](crate::val::Value) is a `false` (Bool)[crate::val::Bool]
    pub fn is_false(&self) -> bool {
        !self.is_true()
    }

    /// Construct a `Bool` 'false' `Value`
    pub fn make_false() -> Value {
        Value::Bool(false.into())
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Construct a `Na` `Value`
    pub fn make_na() -> Value {
        Value::Na
    }

    /// True if this `Value` is a haystack `Na`
    pub fn is_na(&self) -> bool {
        matches!(self, Value::Na)
    }

    /// Construct a `Number` `Value` from a floating point value
    pub fn make_number(value: f64) -> Value {
        Value::from(value)
    }

    /// Construct a `Number` `Value` from a integer value
    pub fn make_int(value: i64) -> Value {
        Value::from(value as f64)
    }

    /// Construct a `Number` `Value` with an unit
    pub fn make_number_unit(value: f64, unit: &'static Unit) -> Value {
        Value::Number(Number::make_with_unit(value, unit))
    }

    /// True if this `Value` is a haystack `Number`
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Construct a `Str` `Value` from a string
    pub fn make_str(value: &str) -> Value {
        Value::from(value)
    }
    /// True if this `Value` is a haystack `Str`
    pub fn is_str(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Construct a `Ref` `Value` from a string
    pub fn make_ref(value: &str) -> Value {
        Value::from(Ref::from(value))
    }

    /// Construct a `Ref` `Value` by generating an unique `Ref`
    pub fn make_ref_gen() -> Value {
        Value::from(Ref::gen())
    }

    /// Construct a `Ref` `Value` from a string with a display name
    pub fn make_ref_with_dis(value: &str, dis: &str) -> Value {
        Value::from(Ref {
            value: String::from(value),
            dis: Some(String::from(dis)),
        })
    }

    /// True if this `Value` is a haystack `Ref`
    pub fn is_ref(&self) -> bool {
        matches!(self, Value::Ref(_))
    }

    /// Construct a `Symbol` `Value` from a string
    pub fn make_symbol(value: &str) -> Value {
        Value::from(Symbol::from(value))
    }

    /// True if this `Value` is a haystack `Symbol`
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }

    /// Construct a `Uri` `Value` from a string
    pub fn make_uri(value: &str) -> Value {
        Value::from(Uri::from(value))
    }

    /// True if this `Value` is a haystack `Uri`
    pub fn is_uri(&self) -> bool {
        matches!(self, Value::Uri(_))
    }

    /// Construct a `Date` `Value` from a `Date`
    pub fn make_date(value: Date) -> Value {
        Value::from(value)
    }

    /// True if this `Value` is a haystack `Date`
    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }

    /// Construct a `Time` `Value` from a `Time`
    pub fn make_time(value: Time) -> Value {
        Value::from(value)
    }

    /// True if this `Value` is a haystack `Time`
    pub fn is_time(&self) -> bool {
        matches!(self, Value::Time(_))
    }

    /// Construct a `DateTime` `Value` from a `DateTime`
    pub fn make_datetime(value: DateTime) -> Value {
        Value::from(value)
    }
    /// Try to construct a `DateTime` `Value` from a string
    pub fn make_datetime_from_iso(value: &str) -> Result<Value, String> {
        let date_time = DateTime::parse_from_rfc3339(value)?;
        Ok(Value::from(date_time))
    }

    /// True if this `Value` is a haystack `DateTime`
    pub fn is_datetime(&self) -> bool {
        matches!(self, Value::DateTime(_))
    }

    /// Construct a `Coord` `Value` from a `Coord`
    pub fn make_coord(value: Coord) -> Value {
        Value::from(value)
    }

    /// Construct a `Coord` `Value` from a latitude and longitude
    pub fn make_coord_from(lat: f64, long: f64) -> Value {
        Value::from(Coord { lat, long })
    }

    /// True if this `Value` is a haystack `Coord`
    pub fn is_coord(&self) -> bool {
        matches!(self, Value::Coord(_))
    }

    /// Construct a `XStr` `Value` from a `XStr`
    pub fn make_xstr(value: XStr) -> Value {
        Value::from(value)
    }

    /// Construct a `XStr` `Value` from a type and value
    pub fn make_xstr_from(typ: &str, value: &str) -> Value {
        Value::from(XStr::make(typ, value))
    }

    /// True if this `Value` is a haystack `XStr`
    pub fn is_xstr(&self) -> bool {
        matches!(self, Value::XStr(_))
    }

    /// Construct a `List` `Value` from a `List`
    pub fn make_list(value: List) -> Value {
        Value::from(value)
    }

    /// True if this `Value` is a haystack `List`
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Construct a `Dict` `Value` from a `Dict`
    pub fn make_dict(value: Dict) -> Value {
        Value::from(value)
    }

    /// True if this `Value` is a haystack `Dict`
    pub fn is_dict(&self) -> bool {
        matches!(self, Value::Dict(_))
    }

    /// Construct a `Grid` `Value` from a `Grid`
    pub fn make_grid(value: Grid) -> Value {
        Value::from(value)
    }

    /// Construct a `Grid` `Value` from a list of `Dict`s
    pub fn make_grid_from_dicts(value: Vec<Dict>) -> Value {
        Value::from(Grid::make_from_dicts(value))
    }

    /// True if this `Value` is a haystack `Dict
    pub fn is_grid(&self) -> bool {
        matches!(self, Value::Grid(_))
    }
}

/// Implements the `Default` trait for the `Value`
impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

/// Implement user friendly display for a [Value](crate::val::Value)
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::haystack::encoding::zinc::encode::ToZinc;
        match self {
            Value::Null => f.write_str("Null"),

            Value::Remove => f.write_str("Remove"),

            Value::Marker => f.write_str("Marker"),

            Value::Bool(val) => f.write_str(if val.value { "true" } else { "false" }),

            Value::Na => f.write_str("Na"),

            Value::Number(_)
            | Value::Str(_)
            | Value::Ref(_)
            | Value::Uri(_)
            | Value::Symbol(_)
            | Value::Date(_)
            | Value::Time(_)
            | Value::DateTime(_)
            | Value::Coord(_)
            | Value::XStr(_)
            | Value::List(_)
            | Value::Dict(_)
            | Value::Grid(_) => match self.to_zinc_string() {
                Ok(zinc) => f.write_str(&zinc),
                Err(_) => Err(std::fmt::Error),
            },
        }
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => Self::Null.hash(state),

            Value::Marker => super::marker::Marker.hash(state),
            Value::Remove => super::remove::Remove.hash(state),
            Value::Na => super::na::Na.hash(state),

            Value::Bool(val) => val.hash(state),
            Value::Number(val) => val.hash(state),
            Value::Str(val) => val.hash(state),
            Value::Ref(val) => val.hash(state),
            Value::Uri(val) => val.hash(state),
            Value::Symbol(val) => val.hash(state),
            Value::Date(val) => val.hash(state),
            Value::Time(val) => val.hash(state),
            Value::DateTime(val) => val.hash(state),
            Value::Coord(val) => val.hash(state),
            Value::XStr(val) => val.hash(state),
            Value::List(val) => val.hash(state),
            Value::Dict(val) => val.hash(state),
            Value::Grid(val) => val.hash(state),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_null(),

            Value::Marker => other.is_marker(),
            Value::Remove => other.is_remove(),
            Value::Na => other.is_na(),

            Value::Bool(val) => matches!(other, Value::Bool(v) if v == val),
            Value::Number(val) => matches!(other, Value::Number(v) if v == val),
            Value::Str(val) => matches!(other, Value::Str(v) if v == val),
            Value::Ref(val) => matches!(other, Value::Ref(v) if v == val),
            Value::Uri(val) => matches!(other, Value::Uri(v) if v == val),
            Value::Symbol(val) => matches!(other, Value::Symbol(v) if v == val),
            Value::Date(val) => matches!(other, Value::Date(v) if v == val),
            Value::Time(val) => matches!(other, Value::Time(v) if v == val),
            Value::DateTime(val) => matches!(other, Value::DateTime(v) if v == val),
            Value::Coord(val) => matches!(other, Value::Coord(v) if v == val),
            Value::XStr(val) => matches!(other, Value::XStr(v) if v == val),
            Value::List(val) => matches!(other, Value::List(v) if v == val),
            Value::Dict(val) => matches!(other, Value::Dict(v) if v == val),
            Value::Grid(val) => matches!(other, Value::Grid(v) if v == val),
        }
    }
}
