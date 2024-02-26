// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! Implement Hayson decoding
//!
use crate::haystack::val::{
    Column, Coord, Date, DateTime, Dict, Grid, HaystackDict, List, Marker, Na, Number, Ref, Remove,
    Str, Symbol, Time, Uri, Value as HVal, XStr,
};

use crate::haystack::timezone::make_date_time_with_tz;
use crate::units::get_unit;
use crate::val::GRID_FORMAT_VERSION;

use chrono::{Offset, Utc};
use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use std::fmt;

/// Hayson Marker deserializer
impl<'de> Deserialize<'de> for Marker {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Marker, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        if val.is_marker() {
            Ok(Marker)
        } else {
            Err(D::Error::custom("Invalid Hayson Marker"))
        }
    }
}

type JsonErr = serde_json::Error;

/// Hayson Remove deserializer
impl<'de> Deserialize<'de> for Remove {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Remove, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        if val.is_remove() {
            Ok(Remove)
        } else {
            Err(D::Error::custom("Invalid Hayson Remove"))
        }
    }
}

/// Hayson Remove deserializer
impl<'de> Deserialize<'de> for Na {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Na, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        if val.is_na() {
            Ok(Na)
        } else {
            Err(D::Error::custom("Invalid Hayson NA"))
        }
    }
}

/// Hayson Number deserializer
impl<'de> Deserialize<'de> for Number {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Number, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Number(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Number")),
        }
    }
}

/// Hayson Date deserializer
impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Date(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Date")),
        }
    }
}

/// Hayson Time deserializer
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Time, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Time(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Time")),
        }
    }
}

/// Hayson DateTime deserializer
impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::DateTime(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson DateTime")),
        }
    }
}

/// Hayson Ref deserializer
impl<'de> Deserialize<'de> for Ref {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Ref, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Ref(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Ref")),
        }
    }
}

/// Hayson Uri deserializer
impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Uri, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Uri(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Uri")),
        }
    }
}

/// Hayson Symbol deserializer
impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Symbol, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Symbol(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Symbol")),
        }
    }
}

/// Hayson Str deserializer
impl<'de> Deserialize<'de> for Str {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Str, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Str(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Str")),
        }
    }
}

/// Hayson Coord deserializer
impl<'de> Deserialize<'de> for Coord {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Coord, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Coord(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Coord")),
        }
    }
}

/// Hayson XStr deserializer
impl<'de> Deserialize<'de> for XStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<XStr, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::XStr(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson XStr")),
        }
    }
}

/// Hayson XStr deserializer
impl<'de> Deserialize<'de> for Dict {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Dict, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Dict(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Dict")),
        }
    }
}

/// Hayson Grid deserializer
impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Grid, D::Error> {
        let val = deserializer.deserialize_any(JsonValueDecoderVisitor)?;
        match val {
            HVal::Grid(val) => Ok(val),
            _ => Err(D::Error::custom("Invalid Hayson Grid")),
        }
    }
}

/// Hayson deserializer
impl<'de> Deserialize<'de> for HVal {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<HVal, D::Error> {
        deserializer.deserialize_any(JsonValueDecoderVisitor)
    }
}

pub struct JsonValueDecoderVisitor;
/// Visitor for Hayson elements
impl<'de> Visitor<'de> for JsonValueDecoderVisitor {
    type Value = HVal;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid Hayson Haystack encoding.")
    }

    /// Null
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(HVal::default())
    }

    /// Bool
    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(HVal::make_bool(v))
    }

    /// Number (integral) primitive
    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v as f64))
    }
    /// Number primitive
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(HVal::make_number(v))
    }

    /// Str
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(HVal::make_str(v))
    }

    /// List
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut list: List = List::new();

        while let Some(_value) = seq.next_element()? {
            let val: HVal = _value;
            list.push(val);
        }

        Ok(HVal::make_list(list))
    }

    /// Objects
    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut dict = Dict::new();
        let mut kind = String::default();

        while let Some((key, value)) = access.next_entry()? {
            if key == "_kind" {
                match value {
                    HVal::Str(str_kind) => {
                        kind = str_kind.value;
                        match kind.as_str() {
                            "marker" => return Ok(HVal::make_marker()),
                            "remove" => return Ok(HVal::make_remove()),
                            "na" => return Ok(HVal::make_na()),
                            // These types are handled by the specialized
                            // `parse_` functions, no need to process them
                            "number" => continue,
                            "ref" => continue,
                            "symbol" => continue,
                            "uri" => continue,
                            "date" => continue,
                            "time" => continue,
                            "dateTime" => continue,
                            "coord" => continue,
                            "xstr" => continue,
                            "grid" => continue,
                            "dict" => continue,
                            _ => {
                                return Err(A::Error::custom(format!(
                                    "Invalid Hayson kind: {kind}"
                                )))
                            }
                        }
                    }
                    _ => return Err(A::Error::custom("Invalid Hayson kind type")),
                }
            }
            dict.insert(key, value);
        }

        match kind.as_str() {
            "number" => match parse_number(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Number. {err}"))),
            },

            "ref" => match parse_ref(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Ref. {err}"))),
            },

            "symbol" => match parse_symbol(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Symbol. {err}"))),
            },

            "uri" => match parse_uri(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Uri. {err}"))),
            },

            "date" => match parse_date(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Date. {err}"))),
            },

            "time" => match parse_time(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Time. {err}"))),
            },

            "dateTime" => match parse_datetime(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson DateTime. {err}"))),
            },

            "coord" => match parse_coord(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Coord. {err}"))),
            },

            "xstr" => match parse_xstr(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson XStr. {err}"))),
            },

            "grid" => match parse_grid(&dict) {
                Ok(val) => Ok(val),
                Err(err) => Err(A::Error::custom(format!("Invalid Hayson Grid. {err}"))),
            },
            _ => Ok(HVal::make_dict(dict)),
        }
    }
}

fn parse_number(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_num("val") {
        Some(val) => match dict.get_str("unit") {
            Some(unit) => {
                if let Some(unit) = get_unit(unit.as_str()) {
                    Ok(Number {
                        value: val.value,
                        unit: Some(unit),
                    }
                    .into())
                } else {
                    Err(JsonErr::custom(format!("Unit not found: {unit:?}")))
                }
            }
            None => Ok((*val).into()),
        },
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_ref(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => match dict.get_str("dis") {
            Some(dis) => Ok(Ref {
                value: val.value.clone(),
                dis: Some(dis.value.clone()),
            }
            .into()),
            None => Ok(Ref {
                value: val.value.clone(),
                dis: None,
            }
            .into()),
        },
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_symbol(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => Ok(HVal::make_symbol(&val.value)),
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_uri(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => Ok(HVal::make_uri(&val.value)),
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_date(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => match val.as_str().parse::<Date>() {
            Ok(date) => Ok(HVal::make_date(date)),
            Err(err) => Err(JsonErr::custom(format!("Invalid date 'val', {err}"))),
        },
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_time(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => match val.as_str().parse::<Time>() {
            Ok(time) => Ok(HVal::make_time(time)),
            Err(err) => Err(JsonErr::custom(format!("Invalid time 'val', {err}"))),
        },
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_datetime(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("val") {
        Some(val) => match DateTime::parse_from_rfc3339(&val.value) {
            Ok(date) => match dict.get_str("tz") {
                Some(tz) => {
                    let datetime =
                        make_date_time_with_tz(&date.with_timezone(&Utc.fix()), &tz.value);
                    match datetime {
                        Ok(datetime) => Ok(HVal::DateTime(datetime.into())),
                        Err(err) => Err(JsonErr::custom(err)),
                    }
                }
                None => Ok(HVal::make_datetime(date)),
            },
            Err(err) => Err(JsonErr::custom(format!("Invalid datetime 'val', {err}"))),
        },
        None => Err(JsonErr::custom("Missing or invalid 'val'")),
    }
}

fn parse_coord(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_num("lat") {
        Some(lat) => match dict.get_num("lng") {
            Some(lng) => Ok(HVal::make_coord_from(lat.value, lng.value)),
            None => Err(JsonErr::custom("Missing or invalid 'lng'")),
        },
        None => Err(JsonErr::custom("Missing or invalid 'lat'")),
    }
}

fn parse_xstr(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_str("type") {
        Some(r#type) => match dict.get_str("val") {
            Some(val) => Ok(HVal::make_xstr_from(&r#type.value, &val.value)),
            None => Err(JsonErr::custom("Missing or invalid 'val'")),
        },
        None => Err(JsonErr::custom("Missing or invalid 'type'")),
    }
}

fn parse_grid(dict: &Dict) -> Result<HVal, JsonErr> {
    match dict.get_list("rows") {
        Some(rows) => match dict.get_list("cols") {
            Some(cols) => {
                let mut grid_ver = GRID_FORMAT_VERSION.to_string();

                let grid = Grid {
                    meta: dict.get_dict("meta").cloned().map(|mut meta| {
                        if let Some(ver) = meta.get_str("ver") {
                            grid_ver = ver.value.to_owned();
                        };
                        meta.remove("ver");
                        meta
                    }),
                    columns: {
                        let cols: Result<Vec<Column>, JsonErr> = cols
                            .iter()
                            .map(|col| match col {
                                HVal::Dict(dict) => match dict.get_str("name") {
                                    Some(name) => match dict.get("meta") {
                                        Some(meta) => match meta {
                                            HVal::Dict(meta) => Ok(Column {
                                                name: name.value.clone(),
                                                meta: Some(meta.clone()),
                                            }),
                                            _ => Err(JsonErr::custom("Invalid 'meta'")),
                                        },
                                        None => Ok(Column {
                                            name: name.value.clone(),
                                            meta: None,
                                        }),
                                    },
                                    None => Err(JsonErr::custom("Missing or invalid 'name'")),
                                },
                                _ => Err(JsonErr::custom("Invalid column type, expected a Dict")),
                            })
                            .collect();
                        cols?
                    },

                    rows: {
                        let rows: Result<Vec<Dict>, JsonErr> = rows
                            .iter()
                            .map(|row| match row {
                                HVal::Dict(row) => Ok(row.clone()),
                                _ => Err(JsonErr::custom("Invalid row type, expected a Dict")),
                            })
                            .collect();
                        rows?
                    },
                    ver: grid_ver,
                };
                Ok(grid.into())
            }
            None => Err(JsonErr::custom("Missing or invalid 'cols'")),
        },
        None => Err(JsonErr::custom("Missing or invalid 'rows'")),
    }
}
