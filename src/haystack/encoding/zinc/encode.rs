// Copyright (C) 2020 - 2022, J2 Innovations

//! Implement Zinc encoding

use crate::haystack::val::{
    Bool, Column, Coord, Date, DateTime, Dict, Grid, List, Marker, Na, Number, Ref, Remove, Str,
    Symbol, Time, Uri, Value, XStr,
};
use chrono::SecondsFormat;
use std::fmt::Display;

/// Zinc encoding version
pub const VER: f32 = 3.0;

/// Zinc encoding trait implemented by scalar and collection types
pub trait ToZinc {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;

    /// Encodes this Haystack type as a Zinc string
    ///
    /// # Example
    /// ```
    /// use libhaystack::val::*;
    /// use libhaystack::encoding::zinc::encode::*;
    /// use libhaystack::units::get_unit_or_default;
    /// let val = Number::make_with_unit(100.0, get_unit_or_default("s"));
    /// assert_eq!(val.to_zinc_string(), Ok("100s".to_string()));
    /// ```
    fn to_zinc_string(&self) -> Result<String> {
        let mut output = Vec::new();
        self.to_zinc(&mut output)?;
        Ok(String::from_utf8(output)?)
    }
}

/// Function that take a haystack  [Value](crate::val::Value)
/// and returns its Zinc string encoding
///
/// # Example
/// ```
/// use libhaystack::val::*;
/// use libhaystack::encoding::zinc::encode::*;
/// let val = Value::make_true();
/// assert_eq!(to_zinc_string(&val), Ok("T".to_string()));
///
pub fn to_zinc_string(value: &Value) -> Result<String> {
    let mut output = Vec::new();
    value.to_zinc(&mut output)?;
    Ok(String::from_utf8(output)?)
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
enum InnerGrid {
    Yes,
    No,
}

/// Specialized trait for encoding inner Grids
trait ZincEncode: ToZinc {
    fn zinc_encode<W: std::io::Write>(&self, writer: &mut W, in_grid: InnerGrid) -> Result<()>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Self {
        Error::from("Format error.")
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::from("IO error.")
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Message(String::from(msg))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::from("Utf8 encoding error.")
    }
}

fn write_str<W: std::io::Write>(writer: &mut W, s: &str) -> Result<()> {
    let bytes = s.as_bytes();
    writer.write_all(bytes)?;
    Ok(())
}

impl ToZinc for Marker {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"M")?;
        Ok(())
    }
}

impl ToZinc for Remove {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"R")?;
        Ok(())
    }
}

impl ToZinc for Na {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"NA")?;
        Ok(())
    }
}

impl ToZinc for Bool {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        if self.value {
            writer.write_all(b"T")?
        } else {
            writer.write_all(b"F")?
        }
        Ok(())
    }
}

impl ToZinc for Number {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        if self.value.is_nan() {
            writer.write_all(b"NaN")?
        } else if self.value.is_infinite() {
            let sign = if self.value.is_sign_negative() {
                "-"
            } else {
                ""
            };
            writer.write_fmt(format_args!("{}INF", sign))?
        } else if let Some(unit) = &self.unit {
            writer.write_fmt(format_args!("{}{}", self.value, unit))?
        } else {
            writer.write_fmt(format_args!("{}", self.value))?
        }
        Ok(())
    }
}

impl ToZinc for Date {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        write_str(writer, &self.to_string())?;
        Ok(())
    }
}

impl ToZinc for Time {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        write_str(writer, &self.to_string())?;
        Ok(())
    }
}

impl ToZinc for DateTime {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        if self.is_utc() {
            write_str(writer, &self.to_rfc3339_opts(SecondsFormat::AutoSi, true))?;
        } else {
            writer.write_fmt(format_args!(
                "{} {}",
                &self.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                &self.timezone_short_name()
            ))?
        }
        Ok(())
    }
}

impl ToZinc for Str {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"\"")?;
        let mut buf = [0; 4];
        for c in self.value.chars() {
            if c < ' ' || c == '"' || c == '\\' {
                match c {
                    '"' => writer.write_all(br#"\""#)?,
                    '\t' => writer.write_all(br#"\t"#)?,
                    '\r' => writer.write_all(br#"\r"#)?,
                    '\n' => writer.write_all(br#"\n"#)?,
                    '\\' => writer.write_all(br#"\\"#)?,
                    _ => writer.write_fmt(format_args!("\\u{:04x}", c as u32))?,
                }
            } else if c == '$' {
                writer.write_all(br#"\$"#)?
            } else {
                let chunk = c.encode_utf8(&mut buf);
                writer.write_fmt(format_args!("{}", chunk))?
            }
        }
        writer.write_all(b"\"")?;
        Ok(())
    }
}

impl ToZinc for Ref {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        if let Some(dis) = &self.dis {
            writer.write_fmt(format_args!("@{} \"{}\"", self.value, dis))?
        } else {
            writer.write_fmt(format_args!("@{}", self.value))?
        }
        Ok(())
    }
}

impl ToZinc for Symbol {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_fmt(format_args!("^{}", self.value))?;
        Ok(())
    }
}

impl ToZinc for Uri {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"`")?;
        for c in self.value.chars() {
            if c < ' ' {
                continue;
            }
            match c {
                '`' => writer.write_all(br#"\`"#)?,
                '\\' => writer.write_all(br#"\\"#)?,
                '\x20'..='\x7e' => writer.write_all(&[c as u8])?,
                _ => writer.write_fmt(format_args!("\\u{:04x}", c as u32))?,
            }
        }
        writer.write_all(b"`")?;
        Ok(())
    }
}

impl ToZinc for XStr {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_fmt(format_args!(
            "{}{}(\"{}\")",
            self.r#type[0..1].to_uppercase(),
            &self.r#type[1..],
            self.value
        ))?;
        Ok(())
    }
}

impl ToZinc for Coord {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_fmt(format_args!("C({},{})", self.lat, self.long))?;
        Ok(())
    }
}

impl ToZinc for Grid {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        self.zinc_encode(writer, InnerGrid::No)
    }
}

impl ZincEncode for Grid {
    fn zinc_encode<W: std::io::Write>(&self, writer: &mut W, in_grid: InnerGrid) -> Result<()> {
        if in_grid == InnerGrid::Yes {
            writer.write_all(b"<<\n")?;
        }

        writer.write_fmt(format_args!("ver:\"{:.1}\"\n", VER))?;

        // Grid meta
        if let Some(meta) = &self.meta {
            write_dict_tags(writer, meta, b" ")?;
        }

        if self.is_empty() {
            // No rows to be written
            writer.write_all(b"empty\n")?;
        } else {
            // Columns
            for (i, col) in self.columns.iter().enumerate() {
                col.to_zinc(writer)?;
                if i < self.columns.len() - 1 {
                    writer.write_all(b",")?;
                }
            }
            writer.write_all(b"\n")?;
            // Rows
            for row in &self.rows {
                // Tags
                for (i, col) in self.columns.iter().enumerate() {
                    if let Some(tag) = row.get(&col.name) {
                        tag.zinc_encode(writer, InnerGrid::Yes)?;
                    }
                    if i < self.columns.len() - 1 {
                        writer.write_all(b",")?;
                    }
                }
                writer.write_all(b"\n")?;
            }
        }
        if in_grid == InnerGrid::Yes {
            writer.write_all(b">>")?;
        } else {
            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}

impl ToZinc for List {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"[")?;
        for (i, el) in self.iter().enumerate() {
            el.zinc_encode(writer, InnerGrid::Yes)?;
            if i < self.len() - 1 {
                writer.write_all(b",")?;
            }
        }
        writer.write_all(b"]")?;
        Ok(())
    }
}

impl ToZinc for Dict {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"{")?;
        write_dict_tags(writer, self, b",")?;
        writer.write_all(b"}")?;
        Ok(())
    }
}

/// Implement the Zinc encoding for Haystack Value type
impl ToZinc for Value {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        self.zinc_encode(writer, InnerGrid::No)
    }
}

/// Implements `ZincEncode` for the `Value` type
/// This implementation deals with nested grids.
impl ZincEncode for Value {
    fn zinc_encode<W: std::io::Write>(&self, writer: &mut W, in_grid: InnerGrid) -> Result<()> {
        match self {
            Value::Null => writer.write_all(b"N")?,

            Value::Remove => Remove.to_zinc(writer)?,

            Value::Marker => Marker.to_zinc(writer)?,

            Value::Bool(val) => val.to_zinc(writer)?,

            Value::Na => Na.to_zinc(writer)?,

            Value::Number(val) => val.to_zinc(writer)?,

            Value::Str(val) => val.to_zinc(writer)?,

            Value::Ref(val) => val.to_zinc(writer)?,

            Value::Uri(val) => val.to_zinc(writer)?,

            Value::Symbol(val) => val.to_zinc(writer)?,

            Value::Date(val) => val.to_zinc(writer)?,

            Value::Time(val) => val.to_zinc(writer)?,

            Value::DateTime(val) => val.to_zinc(writer)?,

            Value::Coord(val) => val.to_zinc(writer)?,

            Value::XStr(val) => val.to_zinc(writer)?,

            Value::List(val) => val.to_zinc(writer)?,

            Value::Dict(val) => val.to_zinc(writer)?,

            Value::Grid(val) => val.zinc_encode(writer, in_grid)?,
        }
        Ok(())
    }
}
/// Serialize a Grid column
impl ToZinc for Column {
    fn to_zinc<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        write_str(writer, &self.name)?;
        if let Some(meta) = &self.meta {
            write_dict_tags(writer, meta, b" ")?;
        }
        Ok(())
    }
}

fn write_dict_tags<W: std::io::Write>(
    writer: &mut W,
    dict: &Dict,
    separator: &[u8; 1],
) -> Result<()> {
    for (pos, (k, v)) in dict.iter().enumerate() {
        write_str(writer, k)?;
        if !v.is_marker() {
            writer.write_all(b":")?;
            v.zinc_encode(writer, InnerGrid::Yes)?;
        }
        if pos < dict.len() - 1 {
            writer.write_all(separator)?;
        }
    }
    Ok(())
}
