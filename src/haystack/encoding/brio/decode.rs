// Copyright (C) 2020 - 2022, J2 Innovations

//! Brio binary decoder for Haystack values.
//!
//! Decodes Brio binary data back into Haystack [`Value`]s.

use std::io::Read;

use crate::haystack::val::{
    Bool, Column, Coord, Date, DateTime, Dict, Grid, List, Marker, Na, Number, Ref, Remove, Symbol,
    Time, Uri, Value, XStr,
};

use super::consts::{get_const, FANTOM_EPOCH_UNIX_SECS};
use super::encode::{
    CTRL_COORD, CTRL_DATE, CTRL_DATETIME_I4, CTRL_DATETIME_I8, CTRL_DICT, CTRL_DICT_EMPTY,
    CTRL_FALSE, CTRL_GRID, CTRL_LIST, CTRL_LIST_EMPTY, CTRL_MARKER, CTRL_NA, CTRL_NULL,
    CTRL_NUMBER_F8, CTRL_NUMBER_I2, CTRL_NUMBER_I4, CTRL_REF_I8, CTRL_REF_STR, CTRL_REMOVE,
    CTRL_STR, CTRL_SYMBOL, CTRL_TIME, CTRL_TRUE, CTRL_URI, CTRL_XSTR,
};

// ---------------------------------------------------------------------------
// Error / Result
// ---------------------------------------------------------------------------

/// Decoding error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Message(String),
    UnexpectedEof,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{msg}"),
            Error::UnexpectedEof => write!(f, "Unexpected end of Brio stream"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::UnexpectedEof {
            Error::UnexpectedEof
        } else {
            Error::Message(err.to_string())
        }
    }
}

/// Result type for Brio decoding.
pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------
// Public trait
// ---------------------------------------------------------------------------

/// Decode a Haystack value from the Brio binary format.
///
/// The inverse of [`ToBrio`]: each implementation reads the ctrl byte for its
/// type followed by the payload, mirroring the structure of
/// [`ToBrio::to_brio`].
///
/// [`ToBrio`]: super::encode::ToBrio
pub trait FromBrio: Sized {
    /// Read the complete Brio encoding of `Self` (ctrl byte + payload) from `reader`.
    fn from_brio<R: Read>(reader: &mut R) -> Result<Self>;
}

// ---------------------------------------------------------------------------
// Low-level read helpers
// ---------------------------------------------------------------------------

fn read_u8<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_i16<R: Read>(reader: &mut R) -> Result<i16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(i16::from_be_bytes(buf))
}

fn read_u16<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

fn read_i32<R: Read>(reader: &mut R) -> Result<i32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_be_bytes(buf))
}

fn read_i64<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}

fn read_f64<R: Read>(reader: &mut R) -> Result<f64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(f64::from_be_bytes(buf))
}

/// Decode a variable-length integer.
pub fn decode_varint<R: Read>(reader: &mut R) -> Result<i64> {
    let v = read_u8(reader)?;
    if v == 0xff {
        return Ok(-1);
    }
    if v & 0x80 == 0 {
        return Ok(v as i64);
    }
    if v & 0xc0 == 0x80 {
        let b1 = read_u8(reader)? as i64;
        return Ok(((v & 0x3f) as i64) << 8 | b1);
    }
    if v & 0xe0 == 0xc0 {
        let b1 = read_u8(reader)? as i64;
        let b23 = read_u16(reader)? as i64;
        return Ok(((v & 0x1f) as i64) << 24 | b1 << 16 | b23);
    }
    if v == 0xe0 {
        return read_i64(reader);
    }
    // 0xe1–0xfe are not produced by encode_varint and have no defined meaning
    Err(Error::Message(format!(
        "Invalid varint leading byte: {v:#x}"
    )))
}

/// Read one CESU-8 byte sequence and return the UTF-16 code unit it encodes.
///
/// | Leading byte      | Width | Decoding                                               |
/// |-------------------|-------|--------------------------------------------------------|
/// | `0x00..0x7F`      | 1     | code unit = byte value                                 |
/// | `0xC0..0xDF`      | 2     | `(b0 & 0x1F) << 6  \| (b1 & 0x3F)`                     |
/// | `0xE0..0xEF`      | 3     | `(b0 & 0x0F) << 12 \| (b1 & 0x3F) << 6 \| (b2 & 0x3F)` |
///
/// The 2-byte sequence `0xC0 0x80` decodes to the null code unit U+0000.
fn read_cesu8_unit<R: Read>(reader: &mut R) -> Result<u16> {
    let first = read_u8(reader)?;
    let unit = if first < 0x80 {
        first as u16
    } else if first & 0xE0 == 0xC0 {
        // 2-byte sequence; 0xC0 0x80 → U+0000
        let second = read_u8(reader)?;
        (((first & 0x1F) as u16) << 6) | ((second & 0x3F) as u16)
    } else if first & 0xF0 == 0xE0 {
        let second = read_u8(reader)?;
        let third = read_u8(reader)?;
        (((first & 0x0F) as u16) << 12) | (((second & 0x3F) as u16) << 6) | ((third & 0x3F) as u16)
    } else {
        return Err(Error::Message(format!(
            "Invalid CESU-8 leading byte: {first:#x}"
        )));
    };
    Ok(unit)
}

/// Decode the inline string payload: `varint(utf16_unit_count)` + CESU-8 bytes.
///
/// Fantom/JVM writes one CESU-8 byte sequence per UTF-16 code unit
/// (`BrioWriter.encodeStrChars`).  Supplementary characters (U+10000+) appear
/// as two consecutive surrogate code units, each with its own CESU-8 sequence
/// (6 bytes total per supplementary character).
///
/// Must be called *after* the inline sentinel (`0xff`) has been consumed.
fn decode_str_chars<R: Read>(reader: &mut R) -> Result<String> {
    let unit_count = decode_varint(reader)?;
    if unit_count < 0 {
        return Err(Error::Message("Negative string unit count".into()));
    }
    let mut units: Vec<u16> = Vec::with_capacity(unit_count as usize);
    for _ in 0..unit_count {
        units.push(read_cesu8_unit(reader)?);
    }
    String::from_utf16(&units).map_err(|e| Error::Message(format!("Invalid UTF-16 sequence: {e}")))
}

/// Decode a Brio-encoded string.
///
/// Reads the string code (varint):
/// - `>= 0` → constant-table lookup via [`get_const`] (0 = empty string `""`)
/// - `-1`   → inline: `varint(char_count)` + CESU-8 bytes
pub fn decode_str<R: Read>(reader: &mut R) -> Result<String> {
    let code = decode_varint(reader)?;
    if code == -1 {
        decode_str_chars(reader)
    } else if code >= 0 {
        get_const(code)
            .map(|s| s.to_owned())
            .ok_or_else(|| Error::Message(format!("Unknown Brio const index: {code}")))
    } else {
        Err(Error::Message(format!("Invalid string code: {code}")))
    }
}

// ---------------------------------------------------------------------------
// Ref id: reconstruct `xxxxxxxx-xxxxxxxx` from packed i64
// ---------------------------------------------------------------------------

fn i8_to_ref_id(val: i64) -> String {
    format!("{:08x}-{:08x}", (val >> 32) as u32, val as u32)
}

// ---------------------------------------------------------------------------
// DateTime construction helpers
// ---------------------------------------------------------------------------

fn datetime_from_secs(unix_secs: i64, tz: &str) -> Result<DateTime> {
    use chrono::{SecondsFormat, TimeZone, Utc};
    let utc = Utc
        .timestamp_opt(unix_secs, 0)
        .single()
        .ok_or_else(|| Error::Message(format!("Invalid unix timestamp: {unix_secs}")))?;
    let rfc3339 = utc.to_rfc3339_opts(SecondsFormat::Secs, true);
    DateTime::parse_from_rfc3339_with_timezone(&rfc3339, tz).map_err(Error::Message)
}

fn datetime_from_nanos(fantom_nanos: i64, tz: &str) -> Result<DateTime> {
    use chrono::{SecondsFormat, TimeZone, Utc};
    let unix_nanos = fantom_nanos + FANTOM_EPOCH_UNIX_SECS * 1_000_000_000;
    let unix_secs = unix_nanos.div_euclid(1_000_000_000);
    let nanos = unix_nanos.rem_euclid(1_000_000_000) as u32;
    let utc = Utc
        .timestamp_opt(unix_secs, nanos)
        .single()
        .ok_or_else(|| Error::Message(format!("Invalid nanosecond timestamp: {fantom_nanos}")))?;
    let rfc3339 = utc.to_rfc3339_opts(SecondsFormat::Nanos, true);
    DateTime::parse_from_rfc3339_with_timezone(&rfc3339, tz).map_err(Error::Message)
}

// ---------------------------------------------------------------------------
// Private payload decoders (ctrl byte already consumed by caller)
// ---------------------------------------------------------------------------

/// Decode a non-empty Dict payload: `'{' varint(count) (key value)* '}'`
fn decode_dict_payload<R: Read>(reader: &mut R) -> Result<Dict> {
    let marker = read_u8(reader)?;
    if marker != b'{' {
        return Err(Error::Message(format!(
            "Expected '{{' after CTRL_DICT, got {marker:#x}"
        )));
    }
    let count = decode_varint(reader)?;
    if count < 0 {
        return Err(Error::Message("Negative dict count".into()));
    }
    let mut dict = Dict::default();
    for _ in 0..count {
        let key = decode_str(reader)?;
        let val = Value::from_brio(reader)?;
        dict.insert(key, val);
    }
    let close = read_u8(reader)?;
    if close != b'}' {
        return Err(Error::Message(format!(
            "Expected '}}' closing CTRL_DICT, got {close:#x}"
        )));
    }
    Ok(dict)
}

/// Decode a non-empty List payload: `'[' varint(count) value* ']'`
fn decode_list_payload<R: Read>(reader: &mut R) -> Result<List> {
    let marker = read_u8(reader)?;
    if marker != b'[' {
        return Err(Error::Message(format!(
            "Expected '[' after CTRL_LIST, got {marker:#x}"
        )));
    }
    let count = decode_varint(reader)?;
    if count < 0 {
        return Err(Error::Message("Negative list count".into()));
    }
    let mut list = List::with_capacity(count as usize);
    for _ in 0..count {
        list.push(Value::from_brio(reader)?);
    }
    let close = read_u8(reader)?;
    if close != b']' {
        return Err(Error::Message(format!(
            "Expected ']' closing CTRL_LIST, got {close:#x}"
        )));
    }
    Ok(list)
}

/// Decode a Grid payload: `'<' varint(ncols) varint(nrows) meta cols... rows... '>'`
fn decode_grid_payload<R: Read>(reader: &mut R) -> Result<Grid> {
    let marker = read_u8(reader)?;
    if marker != b'<' {
        return Err(Error::Message(format!(
            "Expected '<' after CTRL_GRID, got {marker:#x}"
        )));
    }
    let num_cols = decode_varint(reader)?;
    let num_rows = decode_varint(reader)?;
    if num_cols < 0 || num_rows < 0 {
        return Err(Error::Message("Negative grid dimensions".into()));
    }
    let meta = dict_or_none(reader)?;
    let mut columns = Vec::with_capacity(num_cols as usize);
    for _ in 0..num_cols {
        let name = decode_str(reader)?;
        let col_meta = dict_or_none(reader)?;
        columns.push(Column {
            name,
            meta: col_meta,
        });
    }
    let mut rows: Vec<Dict> = Vec::with_capacity(num_rows as usize);
    for _ in 0..num_rows {
        let mut row = Dict::default();
        for col in &columns {
            let val = Value::from_brio(reader)?;
            if !matches!(val, Value::Null) {
                row.insert(col.name.clone(), val);
            }
        }
        rows.push(row);
    }
    let close = read_u8(reader)?;
    if close != b'>' {
        return Err(Error::Message(format!(
            "Expected '>' closing CTRL_GRID, got {close:#x}"
        )));
    }
    Ok(Grid {
        meta,
        columns,
        rows,
        ver: crate::haystack::val::grid::GRID_FORMAT_VERSION.to_string(),
    })
}

/// Read the next value expecting a Dict; return `None` for an empty Dict.
fn dict_or_none<R: Read>(reader: &mut R) -> Result<Option<Dict>> {
    match Value::from_brio(reader)? {
        Value::Dict(d) if d.is_empty() => Ok(None),
        Value::Dict(d) => Ok(Some(d)),
        _ => Err(Error::Message("Expected a Dict".into())),
    }
}

// ---------------------------------------------------------------------------
// FromBrio implementations
// ---------------------------------------------------------------------------

/// `Dict`, `List`, and `Grid` have standalone `FromBrio` impls because they
/// are useful independently (e.g. when a caller knows the field type without
/// inspecting a ctrl byte).  All scalar and singleton types are decoded only
/// through `Value::from_brio`, which is the natural entry-point for
/// ctrl-byte-driven stream decoding.
impl FromBrio for Dict {
    fn from_brio<R: Read>(reader: &mut R) -> Result<Self> {
        match read_u8(reader)? {
            CTRL_DICT_EMPTY => Ok(Dict::default()),
            CTRL_DICT => decode_dict_payload(reader),
            other => Err(Error::Message(format!(
                "Expected Dict ctrl byte, got {other:#x}"
            ))),
        }
    }
}

impl FromBrio for List {
    fn from_brio<R: Read>(reader: &mut R) -> Result<Self> {
        match read_u8(reader)? {
            CTRL_LIST_EMPTY => Ok(List::default()),
            CTRL_LIST => decode_list_payload(reader),
            other => Err(Error::Message(format!(
                "Expected List ctrl byte, got {other:#x}"
            ))),
        }
    }
}

impl FromBrio for Grid {
    fn from_brio<R: Read>(reader: &mut R) -> Result<Self> {
        match read_u8(reader)? {
            CTRL_GRID => decode_grid_payload(reader),
            other => Err(Error::Message(format!(
                "Expected CTRL_GRID, got {other:#x}"
            ))),
        }
    }
}

impl FromBrio for Value {
    fn from_brio<R: Read>(reader: &mut R) -> Result<Self> {
        let ctrl = read_u8(reader)?;
        match ctrl {
            CTRL_NULL => Ok(Value::Null),
            CTRL_MARKER => Ok(Value::from(Marker)),
            CTRL_NA => Ok(Value::from(Na)),
            CTRL_REMOVE => Ok(Value::from(Remove)),
            CTRL_FALSE => Ok(Value::from(Bool::from(false))),
            CTRL_TRUE => Ok(Value::from(Bool::from(true))),
            CTRL_NUMBER_I2 => {
                let v = read_i16(reader)? as f64;
                let u = decode_str(reader)?;
                Ok(Value::from(make_number(v, &u)))
            }
            CTRL_NUMBER_I4 => {
                let v = read_i32(reader)? as f64;
                let u = decode_str(reader)?;
                Ok(Value::from(make_number(v, &u)))
            }
            CTRL_NUMBER_F8 => {
                let v = read_f64(reader)?;
                let u = decode_str(reader)?;
                Ok(Value::from(make_number(v, &u)))
            }
            CTRL_STR => Ok(Value::from(decode_str(reader)?.as_str())),
            CTRL_REF_STR => {
                let id = decode_str(reader)?;
                let dis = decode_str_chars(reader)?;
                Ok(Value::from(Ref::make(&id, non_empty(dis.as_str()))))
            }
            CTRL_REF_I8 => {
                let id = i8_to_ref_id(read_i64(reader)?);
                let dis = decode_str_chars(reader)?;
                Ok(Value::from(Ref::make(&id, non_empty(dis.as_str()))))
            }
            CTRL_URI => Ok(Value::from(Uri {
                value: decode_str(reader)?,
            })),
            CTRL_DATE => {
                let year = read_i16(reader)? as i32;
                let month = read_u8(reader)? as u32;
                let day = read_u8(reader)? as u32;
                Date::from_ymd(year, month, day)
                    .map(Value::from)
                    .map_err(Error::Message)
            }
            CTRL_TIME => {
                let millis = read_i32(reader)? as u32;
                let hour = millis / 3_600_000;
                let minute = (millis % 3_600_000) / 60_000;
                let second = (millis % 60_000) / 1_000;
                let milli = millis % 1_000;
                Time::from_hms_milli(hour, minute, second, milli)
                    .map(Value::from)
                    .map_err(Error::Message)
            }
            CTRL_DATETIME_I4 => {
                let unix_secs = read_i32(reader)? as i64 + FANTOM_EPOCH_UNIX_SECS;
                let tz = decode_str(reader)?;
                datetime_from_secs(unix_secs, &tz).map(Value::from)
            }
            CTRL_DATETIME_I8 => {
                let fantom_nanos = read_i64(reader)?;
                let tz = decode_str(reader)?;
                datetime_from_nanos(fantom_nanos, &tz).map(Value::from)
            }
            CTRL_COORD => {
                // Fantom packs as (lat+90)*1e6 and (lng+180)*1e6; undo that offset
                let lat = read_i32(reader)? as f64 / 1_000_000.0 - 90.0;
                let lng = read_i32(reader)? as f64 / 1_000_000.0 - 180.0;
                Ok(Value::from(Coord::make(lat, lng)))
            }
            CTRL_XSTR => Ok(Value::from(XStr::make(
                &decode_str(reader)?,
                &decode_str(reader)?,
            ))),
            CTRL_SYMBOL => Ok(Value::from(Symbol::make(&decode_str(reader)?))),
            CTRL_DICT_EMPTY => Ok(Value::from(Dict::default())),
            CTRL_DICT => decode_dict_payload(reader).map(Value::from),
            CTRL_LIST_EMPTY => Ok(Value::from(List::default())),
            CTRL_LIST => decode_list_payload(reader).map(Value::from),
            CTRL_GRID => decode_grid_payload(reader).map(Value::from),
            other => Err(Error::Message(format!(
                "Unknown Brio control byte: {other:#x}"
            ))),
        }
    }
}

// ---------------------------------------------------------------------------
// Public decode entry-point
// ---------------------------------------------------------------------------

/// Decode one Haystack [`Value`] from `reader`.
pub fn from_brio<R: Read>(reader: &mut R) -> Result<Value> {
    Value::from_brio(reader)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_number(v: f64, unit: &str) -> Number {
    if unit.is_empty() {
        Number::make(v)
    } else {
        use crate::units::get_unit_or_default;
        Number::make_with_unit(v, get_unit_or_default(unit))
    }
}

/// Return `Some(s)` if `s` is non-empty, otherwise `None`.
fn non_empty(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict;
    use crate::encoding::brio::encode::ToBrio;
    use crate::haystack::val::*;
    use crate::units::get_unit_or_default;

    fn round_trip(v: &Value) -> Value {
        let bytes = v.to_brio_vec().expect("encode");
        from_brio(&mut bytes.as_slice()).expect("decode")
    }

    #[test]
    fn test_null() {
        assert_eq!(round_trip(&Value::Null), Value::Null);
    }

    #[test]
    fn test_marker() {
        assert_eq!(round_trip(&Value::make_marker()), Value::make_marker());
    }

    #[test]
    fn test_na() {
        assert_eq!(round_trip(&Value::make_na()), Value::make_na());
    }

    #[test]
    fn test_remove() {
        assert_eq!(round_trip(&Value::make_remove()), Value::make_remove());
    }

    #[test]
    fn test_bool() {
        assert_eq!(round_trip(&Value::from(true)), Value::from(true));
        assert_eq!(round_trip(&Value::from(false)), Value::from(false));
    }

    #[test]
    fn test_number_i2() {
        let v = Value::from(Number::make(42.0));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_number_i4() {
        let v = Value::from(Number::make(100_000.0));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_number_f8() {
        let v = Value::from(Number::make(std::f64::consts::PI));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_number_with_unit() {
        let v = Value::from(Number::make_with_unit(100.0, get_unit_or_default("kW")));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_number_negative() {
        let v = Value::from(Number::make(-273.15));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_number_nan() {
        // NaN != NaN so we can't use round_trip with assert_eq; extract the f64 directly.
        let v = Value::from(Number::make(f64::NAN));
        let bytes = v.to_brio_vec().expect("encode");
        let got = from_brio(&mut bytes.as_slice()).expect("decode");
        let n = Number::try_from(&got).expect("expected a Number");
        assert!(n.value.is_nan(), "expected NaN, got {}", n.value);
    }

    #[test]
    fn test_number_inf() {
        let v = Value::from(Number::make(f64::INFINITY));
        let bytes = v.to_brio_vec().expect("encode");
        let got = from_brio(&mut bytes.as_slice()).expect("decode");
        let n = Number::try_from(&got).expect("expected a Number");
        assert!(n.value.is_infinite() && n.value.is_sign_positive());
    }

    #[test]
    fn test_number_neg_inf() {
        let v = Value::from(Number::make(f64::NEG_INFINITY));
        let bytes = v.to_brio_vec().expect("encode");
        let got = from_brio(&mut bytes.as_slice()).expect("decode");
        let n = Number::try_from(&got).expect("expected a Number");
        assert!(n.value.is_infinite() && n.value.is_sign_negative());
    }

    #[test]
    fn test_str_empty() {
        let v = Value::from("");
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_str_ascii() {
        let v = Value::from("hello world");
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_str_unicode() {
        let v = Value::from("héllo wörld 🌍");
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_uri() {
        let v = Value::from(Uri {
            value: "https://project-haystack.org".into(),
        });
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_ref_str_no_dis() {
        let v = Value::from(Ref::make("abc123", None));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_ref_str_with_dis() {
        let v = Value::from(Ref::make("abc123", Some("My Ref")));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_ref_i8_format() {
        // 17-char ref: gets packed to i64
        let v = Value::from(Ref::make("cafebabe-deadbeef", None));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_ref_i8_with_dis() {
        let v = Value::from(Ref::make("cafebabe-deadbeef", Some("Some Entity")));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_date() {
        let d = Date::from_ymd(2021, 6, 15).unwrap();
        let v = Value::from(d);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_time() {
        let t = Time::from_hms_milli(10, 30, 45, 123).unwrap();
        let v = Value::from(t);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_time_midnight() {
        let t = Time::from_hms(0, 0, 0).unwrap();
        let v = Value::from(t);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_datetime_utc() {
        let dt = DateTime::parse_from_rfc3339("2021-06-15T12:00:00Z").unwrap();
        let v = Value::from(dt);
        let got = round_trip(&v);
        // Compare by RFC3339 string since tz representation may differ
        assert_eq!(
            DateTime::try_from(&got).unwrap().timestamp(),
            DateTime::try_from(&v).unwrap().timestamp()
        );
    }

    #[test]
    fn test_coord() {
        let c = Coord::make(45.123456, -73.654321);
        let v = Value::from(c);
        let got = round_trip(&v);
        let got_c = Coord::try_from(&got).unwrap();
        assert!((got_c.lat - c.lat).abs() < 1e-6);
        assert!((got_c.long - c.long).abs() < 1e-6);
    }

    #[test]
    fn test_coord_trivial() {
        let c = Coord::make(0.0, 0.0);
        let v = Value::from(c);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_xstr() {
        let v = Value::from(XStr::make("Blob", "aGVsbG8="));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_symbol() {
        let v = Value::from(Symbol::make("myTag"));
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_empty_dict() {
        let v = Value::from(Dict::default());
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_dict_single_entry() {
        let v = Value::make_dict(dict! { "marker" => Value::make_marker() });
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_dict_mixed() {
        let v = Value::make_dict(dict! {
            "site"  => Value::make_marker(),
            "dis"   => Value::from("Main Campus"),
            "area"  => Value::from(Number::make_with_unit(42_000.0, get_unit_or_default("ft²")))
        });
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_empty_list() {
        let v = Value::from(List::default());
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_list_mixed() {
        let v = Value::from(vec![
            Value::make_marker(),
            Value::from(true),
            Value::from(Number::make(42.0)),
            Value::from("hello"),
        ]);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_grid_empty() {
        let g = Grid::make_empty();
        let v = Value::from(g);
        let got = round_trip(&v);
        let got_g = Grid::try_from(&got).unwrap();
        assert!(got_g.rows.is_empty());
    }

    #[test]
    fn test_grid_single_row() {
        let g = Grid::make_from_dicts(vec![dict! {
            "dis" => Value::from("Site A"),
            "area" => Value::from(Number::make(1000.0))
        }]);
        let v = Value::from(g);
        let got = round_trip(&v);
        let got_g = Grid::try_from(&got).unwrap();
        assert_eq!(got_g.rows.len(), 1);
    }

    #[test]
    fn test_grid_multiple_rows() {
        let g = Grid::make_from_dicts(vec![
            dict! { "dis" => Value::from("First"),  "val" => Value::from(Number::make(1.0)) },
            dict! { "dis" => Value::from("Second"), "val" => Value::from(Number::make(2.0)) },
        ]);
        let v = Value::from(g);
        let got = round_trip(&v);
        let got_g = Grid::try_from(&got).unwrap();
        assert_eq!(got_g.rows.len(), 2);
    }

    #[test]
    fn test_grid_with_meta() {
        let meta = dict! {
            "dis"  => Value::from("My Grid"),
            "site" => Value::make_marker()
        };

        let g = Grid::make_from_dicts_with_meta(
            vec![dict! { "dis" => Value::from("Row 1"), "val" => Value::from(Number::make(42.0)) }],
            meta.clone(),
        );

        let v = Value::from(g);
        let got = round_trip(&v);
        let got_g = Grid::try_from(&got).unwrap();

        assert_eq!(got_g.rows.len(), 1);
        assert_eq!(got_g.meta, Some(meta));
    }

    #[test]
    fn test_grid_with_column_meta() {
        let g = Grid {
            meta: None,
            columns: vec![
                Column {
                    name: "dis".to_string(),
                    meta: Some(dict! { "doc" => Value::from("Display name") }),
                },
                Column {
                    name: "val".to_string(),
                    meta: Some(
                        dict! { "doc" => Value::from("Numeric value"), "unit" => Value::from("kW") },
                    ),
                },
            ],
            rows: vec![dict! {
                "dis" => Value::from("Site A"),
                "val" => Value::from(Number::make_with_unit(100.0, get_unit_or_default("kW")))
            }],
            ver: GRID_FORMAT_VERSION.to_string(),
        };

        let v = Value::from(g.clone());
        let got = round_trip(&v);
        let got_g = Grid::try_from(&got).unwrap();

        assert_eq!(got_g.rows.len(), 1);
        // Verify column meta survived the round-trip
        let dis_col = got_g.columns.iter().find(|c| c.name == "dis").unwrap();

        assert_eq!(
            dis_col.meta,
            Some(dict! { "doc" => Value::from("Display name") })
        );

        let val_col = got_g.columns.iter().find(|c| c.name == "val").unwrap();
        assert_eq!(
            val_col.meta,
            Some(dict! { "doc" => Value::from("Numeric value"), "unit" => Value::from("kW") })
        );
    }

    #[test]
    fn test_nested_dict_in_list() {
        let inner = Value::make_dict(dict! { "x" => Value::from(Number::make(1.0)) });
        let v = Value::from(vec![inner]);
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_varint_round_trip() {
        for n in [
            0i64,
            1,
            0x7f,
            0x80,
            0x3fff,
            0x4000,
            0x1fff_ffff,
            0x2000_0000,
            -1,
        ] {
            let mut buf = Vec::new();
            crate::encoding::brio::encode::encode_varint(&mut buf, n).unwrap();
            let decoded = decode_varint(&mut buf.as_slice()).unwrap();
            assert_eq!(decoded, n, "varint round-trip for {n}");
        }
    }

    #[test]
    fn test_decode_const_str() {
        // varint(185) = [0x80, 0xb9] → "dis"
        let bytes: Vec<u8> = vec![0x80, 0xb9];
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "dis");
    }

    #[test]
    fn test_decode_const_timezone() {
        // varint(24) = [0x18] → "UTC"
        let bytes: Vec<u8> = vec![0x18];
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "UTC");
    }

    #[test]
    fn test_decode_const_unit_kw() {
        // varint(630) = 0x8000|630 = 0x8276 → [0x82, 0x76] → "kW"
        let bytes: Vec<u8> = vec![0x82, 0x76];
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "kW");
    }

    #[test]
    fn test_decode_inline_str() {
        // varint(-1) = 0xff, then varint(5), then "hello"
        let mut bytes: Vec<u8> = vec![0xff, 5];
        bytes.extend_from_slice(b"hello");
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_decode_invalid_const_index() {
        // varint(9999) should return an error — not a valid const index
        let bytes: Vec<u8> = vec![0xc0, 0x00, 0x27, 0x0f]; // varint(9999)
        let result = decode_str(&mut bytes.as_slice());
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_varint_invalid_leading_byte() {
        // 0xe1–0xfe are never written by encode_varint; decoding them must return an error
        // rather than silently consuming 8 bytes as if they were 0xe0.
        for byte in [0xe1u8, 0xf0, 0xfe] {
            let result = decode_varint(&mut [byte].as_ref());
            assert!(
                result.is_err(),
                "expected error for leading byte {byte:#x}, got Ok"
            );
        }
    }

    #[test]
    fn test_round_trip_uses_consts() {
        // Dict with known const tag names should encode compactly and round-trip.
        let v = Value::make_dict(dict! {
            "dis"     => Value::from("Main Campus"),
            "site"    => Value::make_marker(),
            "siteRef" => Value::from(Ref::make("cafebabe-deadbeef", None)),
            "ts"      => Value::from(DateTime::parse_from_rfc3339("2021-06-15T12:00:00Z").unwrap())
        });
        assert_eq!(round_trip(&v), v);
    }

    #[test]
    fn test_decode_str_null_char() {
        // 0xC0 0x80 is the CESU-8 / Modified-UTF-8 encoding of U+0000.
        // varint(-1)=0xff, varint(1)=0x01, then the two-byte null sequence.
        let bytes: Vec<u8> = vec![0xff, 0x01, 0xC0, 0x80];
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "\0");
    }

    #[test]
    fn test_decode_str_emoji() {
        // U+1F600 (😀) encoded as surrogate pair in CESU-8:
        //   high 0xD83D → [0xED, 0xA0, 0xBD]
        //   low  0xDE00 → [0xED, 0xB8, 0x80]
        // varint(-1)=0xff, varint(2)=0x02, then 6 bytes.
        let bytes: Vec<u8> = vec![0xff, 0x02, 0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80];
        let s = decode_str(&mut bytes.as_slice()).unwrap();
        assert_eq!(s, "\u{1F600}");
    }

    // -----------------------------------------------------------------------
    // Haxall interop: round-trip tests derived from BrioTest.fan verifyIO()
    //
    // verifyIO(val, size) in BrioTest.fan both checks the encoded byte count
    // AND decodes back, asserting the reconstructed value equals the original.
    // The tests below cover the decode half of every verifyIO call.
    // -----------------------------------------------------------------------

    #[test]
    fn test_haxall_roundtrip_scalars() {
        // verifyIO(null,1) verifyIO(Marker.val,1) verifyIO(NA.val,1)
        // verifyIO(None.val,1) verifyIO(true,1) verifyIO(false,1)
        assert_eq!(round_trip(&Value::Null), Value::Null);
        assert_eq!(round_trip(&Value::make_marker()), Value::make_marker());
        assert_eq!(round_trip(&Value::make_na()), Value::make_na());
        assert_eq!(round_trip(&Value::make_remove()), Value::make_remove());
        assert_eq!(round_trip(&Value::from(true)), Value::from(true));
        assert_eq!(round_trip(&Value::from(false)), Value::from(false));
    }

    #[test]
    fn test_haxall_roundtrip_numbers() {
        // All n() cases from BrioTest.fan testIO.
        // Note: n(123_456_789, "_foo") is NOT included here because unknown unit
        // symbols are resolved through the unit database on decode, so a synthetic
        // unit like "_foo" does not survive the round-trip (encode size is tested
        // separately in test_haxall_compat_number_nonconstunit).
        let cases: Vec<Value> = vec![
            Value::from(Number::make(12.0)),
            Value::from(Number::make(123_456_789.0)),
            Value::from(Number::make_with_unit(
                123_456_789.0,
                get_unit_or_default("°F"),
            )),
            Value::from(Number::make_with_unit(
                123_456.789,
                get_unit_or_default("°F"),
            )),
            Value::from(Number::make(32767.0)),
            Value::from(Number::make(32768.0)),
            Value::from(Number::make(-32767.0)),
            Value::from(Number::make(-32768.0)),
            Value::from(Number::make(2_147_483_647.0)),
            Value::from(Number::make(2_147_483_648.0)),
            Value::from(Number::make(-2_147_483_648.0)),
            Value::from(Number::make(-2_147_483_649.0)),
        ];
        for v in &cases {
            assert_eq!(round_trip(v), *v, "round-trip failed for {v:?}");
        }
    }

    #[test]
    fn test_haxall_roundtrip_strings() {
        // verifyIO("",2) verifyIO("hello °F world!",19)
        // verifyIO("siteRef",3) verifyIO("New_York",2)
        for s in ["", "hello °F world!", "siteRef", "New_York"] {
            let v = Value::from(s);
            assert_eq!(round_trip(&v), v, "round-trip failed for {s:?}");
        }
    }

    #[test]
    fn test_haxall_roundtrip_refs() {
        // verifyIO(Ref("1deb31b8-7508b187"), 10)  — I8 form, no dis
        let r1 = Value::from(Ref::make("1deb31b8-7508b187", None));
        assert_eq!(round_trip(&r1), r1);
        // verifyIO(Ref("1debX1b8-7508b187"), 21)  — non-hex id, STR form
        let r2 = Value::from(Ref::make("1debX1b8-7508b187", None));
        assert_eq!(round_trip(&r2), r2);
        // verifyIO(Ref("1deb31b8.7508b187"), 21)  — '.' separator, STR form
        let r3 = Value::from(Ref::make("1deb31b8.7508b187", None));
        assert_eq!(round_trip(&r3), r3);
        // verifyIO(Ref("1deb31b8-7508b187", "hi!"), 13)  — I8 form with dis
        let r4 = Value::from(Ref::make("1deb31b8-7508b187", Some("hi!")));
        assert_eq!(round_trip(&r4), r4);
    }

    #[test]
    fn test_haxall_roundtrip_symbols() {
        // verifyIO(Symbol("coolingTower"), 3)  verifyIO(Symbol("foo-bar"), 10)
        let s1 = Value::from(Symbol::make("coolingTower"));
        assert_eq!(round_trip(&s1), s1);
        let s2 = Value::from(Symbol::make("foo-bar"));
        assert_eq!(round_trip(&s2), s2);
    }

    #[test]
    fn test_haxall_roundtrip_datetimes() {
        fn dt(iso: &str, tz: &str) -> Value {
            Value::from(DateTime::parse_from_rfc3339_with_timezone(iso, tz).unwrap())
        }
        let cases = vec![
            dt("2015-11-30T12:03:57-05:00", "New_York"), // I4, const tz
            dt("2015-11-30T12:02:33.378-05:00", "New_York"), // I8, const tz
            dt("2015-11-30T12:03:57.000123-05:00", "New_York"), // I8, µs precision
            dt("2000-01-01T00:00:00+01:00", "Warsaw"),   // I4, non-const tz
            dt("2000-01-01T00:00:00.832+01:00", "Warsaw"), // I8, non-const tz
            dt("1999-06-07T01:02:00-04:00", "New_York"), // I4, pre-2000
            dt("1950-06-07T01:02:00-04:00", "New_York"), // I4, pre-1970
            dt("1950-06-07T01:02:00.123-04:00", "New_York"), // I8, pre-1970
        ];
        for v in &cases {
            assert_eq!(round_trip(v), *v, "round-trip failed for {v:?}");
        }
    }

    #[test]
    fn test_haxall_roundtrip_coord() {
        // verifyIO(Coord(37.54f, 77.43f), 9)  verifyIO(Coord(-17.535f, -149.569f), 9)
        // Floating-point rounding: (lat+90)*1e6 integer packing may change the last
        // bit of the f64 on decode, so we compare with 1e-6 tolerance.
        let c1_orig = Coord::make(37.54, 77.43);
        let c1_rt = Coord::try_from(&round_trip(&Value::from(c1_orig))).unwrap();
        assert!((c1_rt.lat - c1_orig.lat).abs() < 1e-6);
        assert!((c1_rt.long - c1_orig.long).abs() < 1e-6);

        let c2_orig = Coord::make(-17.535, -149.569);
        let c2_rt = Coord::try_from(&round_trip(&Value::from(c2_orig))).unwrap();
        assert!((c2_rt.lat - c2_orig.lat).abs() < 1e-6);
        assert!((c2_rt.long - c2_orig.long).abs() < 1e-6);
    }

    #[test]
    fn test_haxall_roundtrip_collections() {
        // verifyIO(Etc.dict0, 1)  verifyIO(Obj?[,], 1)
        let empty_dict = Value::from(Dict::default());
        assert_eq!(round_trip(&empty_dict), empty_dict);
        let empty_list = Value::from(vec![] as List);
        assert_eq!(round_trip(&empty_list), empty_list);
        // verifyIO(["a":n(2),"b":n(1.2,"kW"),"c":n(123456789,"°F"),"d":n(-3,"_foo")])
        // ("d" with "_foo" unit omitted — unknown units cannot survive decode;
        //  see encode.rs test_haxall_compat_number_nonconstunit for the size check)
        let units_dict = Value::make_dict(dict! {
            "a" => Value::from(Number::make(2.0)),
            "b" => Value::from(Number::make_with_unit(1.2, get_unit_or_default("kW"))),
            "c" => Value::from(Number::make_with_unit(123_456_789.0, get_unit_or_default("°F")))
        });
        assert_eq!(round_trip(&units_dict), units_dict);
    }

    #[test]
    fn test_haxall_roundtrip_mixed_dict() {
        // verifyIO(all-different-types dict) from BrioTest.fan
        let v = Value::make_dict(dict! {
            "m"  => Value::make_marker(),
            "na" => Value::make_na(),
            "bf" => Value::from(false),
            "bt" => Value::from(true),
            "n"  => Value::from(Number::make(123.0)),
            "s"  => Value::from("hi"),
            "r"  => Value::from(Ref::make("1deb31b8-7508b187", None)),
            "u"  => Value::from(Uri { value: "a/b".to_string() }),
            "d"  => Value::from(Date::from_ymd(2021, 6, 15).unwrap()),
            "dt" => Value::from(DateTime::parse_from_rfc3339("2021-06-15T12:00:00Z").unwrap())
        });
        assert_eq!(round_trip(&v), v);
    }
}
