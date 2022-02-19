// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Kinds

use std::fmt::Display;

use super::Value;

/// List all haystack kind types as an simple enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HaystackKind {
    /// No value
    Null,
    /// A remove tag
    Remove,
    /// Marker tag
    Marker,
    /// Na tag
    Na,
    /// Bool true/false
    Bool,
    /// Number floating point value and optional unit
    Number,
    // String value
    Str,
    /// URI with string value
    Uri,
    /// Ref with string value
    Ref,
    /// A symbol with string value
    Symbol,
    /// Date year, month, date
    Date,
    /// Time hour, minutes, seconds with optional millis
    Time,
    /// DateTime date, time and timezone
    DateTime,
    /// Coordinate latitude and longitude
    Coord,
    /// XStr with type and value
    XStr,
    /// List of tags
    List,
    /// Dictionary of `String` key and `Value` values
    Dict,
    /// Haystack Grid
    Grid,
}

/// Default Kind variant
impl Default for HaystackKind {
    fn default() -> Self {
        HaystackKind::Null
    }
}

impl TryFrom<u8> for HaystackKind {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            variant if variant == HaystackKind::Null as u8 => Ok(HaystackKind::Null),
            variant if variant == HaystackKind::Remove as u8 => Ok(HaystackKind::Remove),
            variant if variant == HaystackKind::Marker as u8 => Ok(HaystackKind::Marker),
            variant if variant == HaystackKind::Na as u8 => Ok(HaystackKind::Na),
            variant if variant == HaystackKind::Bool as u8 => Ok(HaystackKind::Bool),
            variant if variant == HaystackKind::Number as u8 => Ok(HaystackKind::Number),
            variant if variant == HaystackKind::Str as u8 => Ok(HaystackKind::Str),
            variant if variant == HaystackKind::Uri as u8 => Ok(HaystackKind::Uri),
            variant if variant == HaystackKind::Ref as u8 => Ok(HaystackKind::Ref),
            variant if variant == HaystackKind::Symbol as u8 => Ok(HaystackKind::Symbol),
            variant if variant == HaystackKind::Date as u8 => Ok(HaystackKind::Date),
            variant if variant == HaystackKind::Time as u8 => Ok(HaystackKind::Time),
            variant if variant == HaystackKind::DateTime as u8 => Ok(HaystackKind::DateTime),
            variant if variant == HaystackKind::Coord as u8 => Ok(HaystackKind::Coord),
            variant if variant == HaystackKind::XStr as u8 => Ok(HaystackKind::XStr),
            variant if variant == HaystackKind::List as u8 => Ok(HaystackKind::List),
            variant if variant == HaystackKind::Dict as u8 => Ok(HaystackKind::Dict),
            variant if variant == HaystackKind::Grid as u8 => Ok(HaystackKind::Grid),

            _ => Err(format!(
                "Invalid variant: '{value}' type for a HaystackKind"
            )),
        }
    }
}

impl From<&Value> for HaystackKind {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => HaystackKind::Null,
            Value::Remove => HaystackKind::Remove,
            Value::Marker => HaystackKind::Marker,
            Value::Na => HaystackKind::Na,
            Value::Bool(_) => HaystackKind::Bool,
            Value::Number(_) => HaystackKind::Number,
            Value::Str(_) => HaystackKind::Str,
            Value::Uri(_) => HaystackKind::Uri,
            Value::Ref(_) => HaystackKind::Ref,
            Value::Symbol(_) => HaystackKind::Symbol,
            Value::Date(_) => HaystackKind::Date,
            Value::Time(_) => HaystackKind::Time,
            Value::DateTime(_) => HaystackKind::DateTime,
            Value::Coord(_) => HaystackKind::Coord,
            Value::XStr(_) => HaystackKind::XStr,
            Value::List(_) => HaystackKind::List,
            Value::Dict(_) => HaystackKind::Dict,
            Value::Grid(_) => HaystackKind::Grid,
        }
    }
}

impl From<HaystackKind> for &'static str {
    fn from(kind: HaystackKind) -> Self {
        match kind {
            HaystackKind::Null => "null",
            HaystackKind::Remove => "remove",
            HaystackKind::Marker => "marker",
            HaystackKind::Na => "na",
            HaystackKind::Bool => "bool",
            HaystackKind::Number => "number",
            HaystackKind::Str => "str",
            HaystackKind::Uri => "uri",
            HaystackKind::Ref => "ref",
            HaystackKind::Symbol => "symbol",
            HaystackKind::Date => "date",
            HaystackKind::Time => "time",
            HaystackKind::DateTime => "dateTime",
            HaystackKind::Coord => "coord",
            HaystackKind::XStr => "xstr",
            HaystackKind::List => "list",
            HaystackKind::Dict => "dict",
            HaystackKind::Grid => "grid",
        }
    }
}

impl TryFrom<&str> for HaystackKind {
    type Error = String;

    fn try_from(kind: &str) -> Result<Self, Self::Error> {
        match kind {
            "null" => Ok(HaystackKind::Null),
            "remove" => Ok(HaystackKind::Remove),
            "marker" => Ok(HaystackKind::Marker),
            "na" => Ok(HaystackKind::Na),
            "bool" => Ok(HaystackKind::Bool),
            "number" => Ok(HaystackKind::Number),
            "str" => Ok(HaystackKind::Str),
            "uri" => Ok(HaystackKind::Uri),
            "ref" => Ok(HaystackKind::Ref),
            "symbol" => Ok(HaystackKind::Symbol),
            "date" => Ok(HaystackKind::Date),
            "time" => Ok(HaystackKind::Time),
            "dateTime" => Ok(HaystackKind::DateTime),
            "coord" => Ok(HaystackKind::Coord),
            "xstr" => Ok(HaystackKind::XStr),
            "list" => Ok(HaystackKind::List),
            "dict" => Ok(HaystackKind::Dict),
            "grid" => Ok(HaystackKind::Grid),
            _ => Err(format!("Invalid kind: {kind}")),
        }
    }
}

impl Display for HaystackKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            HaystackKind::Null => "null",
            HaystackKind::Remove => "remove",
            HaystackKind::Marker => "marker",
            HaystackKind::Na => "na",
            HaystackKind::Bool => "bool",
            HaystackKind::Number => "number",
            HaystackKind::Str => "str",
            HaystackKind::Uri => "uri",
            HaystackKind::Ref => "ref",
            HaystackKind::Symbol => "symbol",
            HaystackKind::Date => "date",
            HaystackKind::Time => "time",
            HaystackKind::DateTime => "dateTime",
            HaystackKind::Coord => "coord",
            HaystackKind::XStr => "xstr",
            HaystackKind::List => "list",
            HaystackKind::Dict => "dict",
            HaystackKind::Grid => "grid",
        };
        write!(fmt, "{kind}")
    }
}
