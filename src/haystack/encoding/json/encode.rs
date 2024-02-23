// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! Implement Hayson encoding
//!

use chrono::SecondsFormat;

use crate::{
    encoding::decode_ref_dis_factory::get_ref_dis_from_factory,
    haystack::val::{
        Column, Coord, Date, DateTime, Dict, Grid, Marker, Na, Number, Ref, Remove, Symbol, Time,
        Uri, Value as HVal, XStr,
    },
};

use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

/// Serialize a Grid column
impl Serialize for Column {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(if self.meta.is_none() { 1 } else { 2 }))?;
        map.serialize_entry("name", &self.name)?;
        if self.meta.is_some() {
            map.serialize_entry("meta", &self.meta)?;
        }
        map.end()
    }
}

impl Serialize for Marker {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("_kind", "marker")?;
        map.end()
    }
}

impl Serialize for Na {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("_kind", "na")?;
        map.end()
    }
}

impl Serialize for Remove {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("_kind", "remove")?;
        map.end()
    }
}

impl Serialize for Ref {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(if self.dis.is_none() { 2 } else { 3 }))?;
        map.serialize_entry("_kind", "ref")?;
        map.serialize_entry("val", &self.value)?;

        let dis = get_ref_dis_from_factory(self.value.as_str(), self.dis.as_deref())
            .map(|v| v.to_string())
            .or_else(|| self.dis.clone());

        if dis.is_some() {
            map.serialize_entry("dis", &dis)?;
        }
        map.end()
    }
}

impl Serialize for Uri {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("_kind", "uri")?;
        map.serialize_entry("val", &self.value)?;
        map.end()
    }
}

impl Serialize for Symbol {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("_kind", "symbol")?;
        map.serialize_entry("val", &self.value)?;
        map.end()
    }
}

impl Serialize for Number {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(unit) = self.unit {
            let mut map = serializer.serialize_map(Some(3))?;
            map.serialize_entry("_kind", "number")?;
            map.serialize_entry("val", &self.value)?;
            map.serialize_entry("unit", unit.symbol())?;
            map.end()
        } else if self.value.fract() == 0.0 {
            serializer.serialize_i64(self.value as i64)
        } else {
            serializer.serialize_f64(self.value)
        }
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("_kind", "date")?;
        map.serialize_entry("val", &self.to_string())?;
        map.end()
    }
}

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("_kind", "time")?;
        map.serialize_entry("val", &self.to_string())?;
        map.end()
    }
}

impl Serialize for DateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("_kind", "dateTime")?;
        map.serialize_entry("val", &self.to_rfc3339_opts(SecondsFormat::AutoSi, true))?;
        if !self.is_utc() {
            map.serialize_entry("tz", &self.timezone_short_name())?;
        }
        map.end()
    }
}

impl Serialize for Coord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("_kind", "coord")?;
        map.serialize_entry("lat", &self.lat)?;
        map.serialize_entry("lng", &self.long)?;
        map.end()
    }
}

impl Serialize for XStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("_kind", "xstr")?;
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("val", &self.value)?;
        map.end()
    }
}

impl Serialize for Dict {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl Serialize for Grid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("_kind", "grid")?;
        if self.meta.is_some() {
            map.serialize_entry("meta", &self.meta)?;
        } else {
            map.serialize_entry("meta", &Dict::new())?;
        }
        map.serialize_entry("cols", &self.columns)?;
        map.serialize_entry("rows", &self.rows)?;
        map.end()
    }
}

///
/// Serializes a haystack value to Haystack JSON (Hayson)
///
impl Serialize for HVal {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            HVal::Null => serializer.serialize_none(),

            HVal::Remove => Remove::serialize(&Remove, serializer),

            HVal::Marker => Marker::serialize(&Marker, serializer),

            HVal::Bool(value) => serializer.serialize_bool(value.value),

            HVal::Na => Na::serialize(&Na, serializer),

            HVal::Number(val) => Number::serialize(val, serializer),

            HVal::Str(val) => serializer.serialize_str(val.value.as_str()),

            HVal::Ref(val) => Ref::serialize(val, serializer),

            HVal::Symbol(val) => Symbol::serialize(val, serializer),

            HVal::Uri(val) => Uri::serialize(val, serializer),

            HVal::Date(val) => Date::serialize(val, serializer),

            HVal::Time(val) => Time::serialize(val, serializer),

            HVal::DateTime(val) => DateTime::serialize(val, serializer),

            HVal::Coord(val) => Coord::serialize(val, serializer),

            HVal::XStr(val) => XStr::serialize(val, serializer),

            HVal::List(val) => {
                let mut seq = serializer.serialize_seq(Some(val.len()))?;
                for el in val {
                    seq.serialize_element(el)?;
                }
                seq.end()
            }

            HVal::Dict(val) => Dict::serialize(val, serializer),

            HVal::Grid(val) => Grid::serialize(val, serializer),
        }
    }
}
