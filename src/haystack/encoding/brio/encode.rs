// Copyright (C) 2020 - 2022, J2 Innovations

//! Brio binary encoder for Haystack values.
//!
//! Encodes all Haystack value types to the Brio binary wire format.

use std::io::Write;

use super::consts::{lookup_const, FANTOM_EPOCH_UNIX_SECS};
use crate::haystack::val::{
    Bool, Coord, Date, DateTime, Dict, Grid, List, Marker, Na, Number, Ref, Remove, Str, Symbol,
    Time, Uri, Value, XStr,
};

// ---------------------------------------------------------------------------
// Control bytes
// ---------------------------------------------------------------------------

pub const CTRL_NULL: u8 = 0x00;
pub const CTRL_MARKER: u8 = 0x01;
pub const CTRL_NA: u8 = 0x02;
pub const CTRL_REMOVE: u8 = 0x03;
pub const CTRL_FALSE: u8 = 0x04;
pub const CTRL_TRUE: u8 = 0x05;
pub const CTRL_NUMBER_I2: u8 = 0x06;
pub const CTRL_NUMBER_I4: u8 = 0x07;
pub const CTRL_NUMBER_F8: u8 = 0x08;
pub const CTRL_STR: u8 = 0x09;
pub const CTRL_REF_STR: u8 = 0x0a;
pub const CTRL_REF_I8: u8 = 0x0b;
pub const CTRL_URI: u8 = 0x0c;
pub const CTRL_DATE: u8 = 0x0d;
pub const CTRL_TIME: u8 = 0x0e;
pub const CTRL_DATETIME_I4: u8 = 0x0f;
pub const CTRL_DATETIME_I8: u8 = 0x10;
pub const CTRL_COORD: u8 = 0x11;
pub const CTRL_XSTR: u8 = 0x12;
pub const CTRL_DICT_EMPTY: u8 = 0x14;
pub const CTRL_DICT: u8 = 0x15;
pub const CTRL_LIST_EMPTY: u8 = 0x16;
pub const CTRL_LIST: u8 = 0x17;
pub const CTRL_GRID: u8 = 0x18;
pub const CTRL_SYMBOL: u8 = 0x19;

// ---------------------------------------------------------------------------
// Error / Result
// ---------------------------------------------------------------------------

/// Encoding error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Message(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Message(err.to_string())
    }
}

/// Result type for Brio encoding.
pub type Result<T> = std::result::Result<T, Error>;

// ---------------------------------------------------------------------------
// Public trait
// ---------------------------------------------------------------------------

/// Encode a Haystack value to the Brio binary format.
pub trait ToBrio {
    /// Write the Brio encoding of `self` into `writer`.
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()>;

    /// Convenience: encode to a `Vec<u8>`.
    fn to_brio_vec(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.to_brio(&mut buf)?;
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// Low-level primitives
// ---------------------------------------------------------------------------

/// Encode a variable-length integer.
///
/// | Range               | Encoding                             |
/// |---------------------|--------------------------------------|
/// | `-1`                | `0xff`                               |
/// | `0..=0x7f`          | 1 byte                               |
/// | `0x80..=0x3fff`     | 2 bytes with `0x8000` mask           |
/// | `0x4000..=0x1fffffff`| 4 bytes with `0xc0000000` mask      |
/// | larger              | `0xe0` + 8 bytes (big-endian `i64`)  |
pub fn encode_varint<W: Write>(writer: &mut W, val: i64) -> Result<()> {
    if val < 0 {
        // Only -1 is valid; treat any negative as -1
        writer.write_all(&[0xff])?;
    } else if val <= 0x7f {
        writer.write_all(&[val as u8])?;
    } else if val <= 0x3fff {
        let encoded = val as u16 | 0x8000;
        writer.write_all(&encoded.to_be_bytes())?;
    } else if val <= 0x1fff_ffff {
        let encoded = val as u32 | 0xc000_0000;
        writer.write_all(&encoded.to_be_bytes())?;
    } else {
        writer.write_all(&[0xe0])?;
        writer.write_all(&val.to_be_bytes())?;
    }
    Ok(())
}

/// Encode a string, preferring a compact constant-table index when available.
///
/// If `s` is found in [`crate::encoding::brio::consts::CONSTS`], writes
/// `varint(index)`.  Otherwise writes the inline form:
/// `varint(-1)` + `varint(utf16_unit_count)` + CESU-8 bytes.
///
/// Fantom/JVM measures string length as the number of UTF-16 code units
/// (`Str.size` = `java.lang.String.length()`) and serialises each unit via
/// `OutStream.writeChar`, which produces CESU-8 on the JVM.  CESU-8 is
/// identical to UTF-8 for all BMP characters **except** U+0000, which is
/// encoded as `0xC0 0x80` rather than a bare null byte.  Supplementary
/// characters (U+10000+) are stored as surrogate pairs, each surrogate
/// encoded as its own 3-byte CESU-8 sequence (6 bytes per character).
pub fn encode_str<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    if let Some(idx) = lookup_const(s) {
        encode_varint(writer, idx)?;
    } else {
        encode_varint(writer, -1)?; // inline sentinel
        encode_str_chars(writer, s)?;
    }
    Ok(())
}

/// Encode `varint(utf16_unit_count)` then one CESU-8 sequence per UTF-16
/// code unit, matching Fantom/JVM `BrioWriter.encodeStrChars`.
fn encode_str_chars<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    let units: Vec<u16> = s.encode_utf16().collect();
    encode_varint(writer, units.len() as i64)?;
    for &unit in &units {
        write_cesu8_unit(writer, unit)?;
    }
    Ok(())
}

/// Write one UTF-16 code unit as its CESU-8 byte sequence.
///
/// | Code unit range   | Bytes | Encoding                                   |
/// |-------------------|-------|--------------------------------------------||
/// | U+0000            | 2     | `0xC0 0x80` (Modified UTF-8 null)          |
/// | U+0001..U+007F    | 1     | same as UTF-8                              |
/// | U+0080..U+07FF    | 2     | same as UTF-8                              |
/// | U+0800..U+FFFF    | 3     | same as UTF-8; covers UTF-16 surrogates    |
fn write_cesu8_unit<W: Write>(writer: &mut W, unit: u16) -> Result<()> {
    match unit {
        0x0000 => writer.write_all(&[0xC0, 0x80])?,
        0x0001..=0x007F => writer.write_all(&[unit as u8])?,
        0x0080..=0x07FF => {
            writer.write_all(&[0xC0 | (unit >> 6) as u8, 0x80 | (unit & 0x3F) as u8])?
        }
        0x0800..=0xFFFF => writer.write_all(&[
            0xE0 | (unit >> 12) as u8,
            0x80 | ((unit >> 6) & 0x3F) as u8,
            0x80 | (unit & 0x3F) as u8,
        ])?,
    }
    Ok(())
}

/// Try to encode a `Ref` id as a packed `i64`.
///
/// Only succeeds for the 17-character format `xxxxxxxx-xxxxxxxx`
/// (8 hex digits, dash, 8 hex digits).
fn ref_id_to_i8(id: &str) -> Option<i64> {
    let bytes = id.as_bytes();
    if bytes.len() != 17 || bytes[8] != b'-' {
        return None;
    }
    let mut val: i64 = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if i == 8 {
            continue;
        }
        let digit = (b as char).to_digit(16)? as i64;
        val = (val << 4) | digit;
    }
    // Fantom's i8 encoding only works for non-negative i64 values;
    // if the packed value overflows (sign bit set), fall back to STR form.
    if val < 0 {
        None
    } else {
        Some(val)
    }
}

// ---------------------------------------------------------------------------
// ToBrio implementations
// ---------------------------------------------------------------------------

impl ToBrio for Value {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Value::Null => writer.write_all(&[CTRL_NULL])?,
            Value::Marker => Marker.to_brio(writer)?,
            Value::Na => Na.to_brio(writer)?,
            Value::Remove => Remove.to_brio(writer)?,
            Value::Bool(v) => v.to_brio(writer)?,
            Value::Number(v) => v.to_brio(writer)?,
            Value::Str(v) => v.to_brio(writer)?,
            Value::Ref(v) => v.to_brio(writer)?,
            Value::Uri(v) => v.to_brio(writer)?,
            Value::Date(v) => v.to_brio(writer)?,
            Value::Time(v) => v.to_brio(writer)?,
            Value::DateTime(v) => v.to_brio(writer)?,
            Value::Coord(v) => v.to_brio(writer)?,
            Value::XStr(v) => v.to_brio(writer)?,
            Value::Symbol(v) => v.to_brio(writer)?,
            Value::List(v) => v.to_brio(writer)?,
            Value::Dict(v) => v.to_brio(writer)?,
            Value::Grid(v) => v.to_brio(writer)?,
        }
        Ok(())
    }
}

impl ToBrio for Marker {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_MARKER])?;
        Ok(())
    }
}

impl ToBrio for Na {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_NA])?;
        Ok(())
    }
}

impl ToBrio for Remove {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_REMOVE])?;
        Ok(())
    }
}

impl ToBrio for Bool {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[if self.value { CTRL_TRUE } else { CTRL_FALSE }])?;
        Ok(())
    }
}

impl ToBrio for Str {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_STR])?;
        encode_str(writer, &self.value)?;
        Ok(())
    }
}

impl ToBrio for Uri {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_URI])?;
        encode_str(writer, &self.value)?;
        Ok(())
    }
}

impl ToBrio for Symbol {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_SYMBOL])?;
        encode_str(writer, &self.value)?;
        Ok(())
    }
}

impl ToBrio for Number {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        let v = self.value;
        let unit_str = self.unit.map_or("", |u| u.symbol());

        // Fantom BrioWriter uses -32767..=32767 (excludes i16::MIN = -32768)
        if v.fract() == 0.0 && v >= -(i16::MAX as f64) && v <= i16::MAX as f64 {
            writer.write_all(&[CTRL_NUMBER_I2])?;
            writer.write_all(&(v as i16).to_be_bytes())?;
        } else if v.fract() == 0.0 && v >= i32::MIN as f64 && v <= i32::MAX as f64 {
            writer.write_all(&[CTRL_NUMBER_I4])?;
            writer.write_all(&(v as i32).to_be_bytes())?;
        } else {
            writer.write_all(&[CTRL_NUMBER_F8])?;
            writer.write_all(&v.to_be_bytes())?;
        }
        encode_str(writer, unit_str)?;
        Ok(())
    }
}

impl ToBrio for Ref {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        let dis = self.dis.as_deref().unwrap_or("");
        if let Some(i8_val) = ref_id_to_i8(&self.value) {
            writer.write_all(&[CTRL_REF_I8])?;
            writer.write_all(&i8_val.to_be_bytes())?;
        } else {
            writer.write_all(&[CTRL_REF_STR])?;
            encode_str(writer, &self.value)?;
        }
        // Fantom BrioWriter.writeRefDis calls encodeStrChars() — always inline,
        // never a const-table code. The matching reader uses decodeStrChars().
        encode_str_chars(writer, dis)?;
        Ok(())
    }
}

impl ToBrio for Date {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        use chrono::Datelike;
        writer.write_all(&[CTRL_DATE])?;
        writer.write_all(&(self.year() as i16).to_be_bytes())?;
        writer.write_all(&[self.month() as u8])?;
        writer.write_all(&[self.day() as u8])?;
        Ok(())
    }
}

impl ToBrio for Time {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        use chrono::Timelike;
        writer.write_all(&[CTRL_TIME])?;
        let millis = self.hour() * 3_600_000
            + self.minute() * 60_000
            + self.second() * 1_000
            + self.nanosecond() / 1_000_000;
        writer.write_all(&(millis as i32).to_be_bytes())?;
        Ok(())
    }
}

impl ToBrio for DateTime {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        let unix_secs = self.timestamp();
        let nanos = self.timestamp_subsec_nanos();
        let fantom_secs = unix_secs - FANTOM_EPOCH_UNIX_SECS;
        let tz = self.timezone_short_name();

        // Use compact I4 when no sub-second precision and secs fit in i32
        if nanos == 0 && fantom_secs >= i32::MIN as i64 && fantom_secs <= i32::MAX as i64 {
            writer.write_all(&[CTRL_DATETIME_I4])?;
            writer.write_all(&(fantom_secs as i32).to_be_bytes())?;
        } else {
            let fantom_nanos = (unix_secs - FANTOM_EPOCH_UNIX_SECS) * 1_000_000_000 + nanos as i64;
            writer.write_all(&[CTRL_DATETIME_I8])?;
            writer.write_all(&fantom_nanos.to_be_bytes())?;
        }
        encode_str(writer, &tz)?;
        Ok(())
    }
}

impl ToBrio for Coord {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_COORD])?;
        // Fantom packs lat as (lat+90)*1e6 and lng as (lng+180)*1e6
        // to keep both values unsigned (all valid lat/lng → positive packed values)
        let pack_lat = ((self.lat + 90.0) * 1_000_000.0_f64).round() as i32;
        let pack_lng = ((self.long + 180.0) * 1_000_000.0_f64).round() as i32;
        writer.write_all(&pack_lat.to_be_bytes())?;
        writer.write_all(&pack_lng.to_be_bytes())?;
        Ok(())
    }
}

impl ToBrio for XStr {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_XSTR])?;
        encode_str(writer, &self.r#type)?;
        encode_str(writer, &self.value)?;
        Ok(())
    }
}

impl ToBrio for Dict {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.is_empty() {
            writer.write_all(&[CTRL_DICT_EMPTY])?;
        } else {
            writer.write_all(&[CTRL_DICT, b'{'])?;
            encode_varint(writer, self.len() as i64)?;
            for (key, val) in self.iter() {
                encode_str(writer, key)?;
                val.to_brio(writer)?;
            }
            writer.write_all(b"}")?;
        }
        Ok(())
    }
}

impl ToBrio for List {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.is_empty() {
            writer.write_all(&[CTRL_LIST_EMPTY])?;
        } else {
            writer.write_all(&[CTRL_LIST, b'['])?;
            encode_varint(writer, self.len() as i64)?;
            for val in self.iter() {
                val.to_brio(writer)?;
            }
            writer.write_all(b"]")?;
        }
        Ok(())
    }
}

impl ToBrio for Grid {
    fn to_brio<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[CTRL_GRID, b'<'])?;
        encode_varint(writer, self.columns.len() as i64)?;
        encode_varint(writer, self.rows.len() as i64)?;

        // Grid meta
        match &self.meta {
            Some(meta) => meta.to_brio(writer)?,
            None => writer.write_all(&[CTRL_DICT_EMPTY])?,
        }

        // Column definitions: name + meta dict
        for col in &self.columns {
            encode_str(writer, &col.name)?;
            match &col.meta {
                Some(meta) => meta.to_brio(writer)?,
                None => writer.write_all(&[CTRL_DICT_EMPTY])?,
            }
        }

        // Row cells (Null for missing columns)
        for row in &self.rows {
            for col in &self.columns {
                match row.get(&col.name) {
                    Some(val) => val.to_brio(writer)?,
                    None => writer.write_all(&[CTRL_NULL])?,
                }
            }
        }

        writer.write_all(b">")?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict;
    use crate::haystack::val::*;
    use crate::units::{get_unit_or_default, Unit};

    fn enc(v: &Value) -> Vec<u8> {
        v.to_brio_vec().expect("encode")
    }

    #[test]
    fn test_null() {
        assert_eq!(enc(&Value::Null), vec![CTRL_NULL]);
    }

    #[test]
    fn test_marker() {
        assert_eq!(enc(&Value::make_marker()), vec![CTRL_MARKER]);
    }

    #[test]
    fn test_na() {
        assert_eq!(enc(&Value::make_na()), vec![CTRL_NA]);
    }

    #[test]
    fn test_remove() {
        assert_eq!(enc(&Value::make_remove()), vec![CTRL_REMOVE]);
    }

    #[test]
    fn test_bool_false() {
        assert_eq!(enc(&Value::from(false)), vec![CTRL_FALSE]);
    }

    #[test]
    fn test_bool_true() {
        assert_eq!(enc(&Value::from(true)), vec![CTRL_TRUE]);
    }

    #[test]
    fn test_number_i2() {
        let v = Value::from(Number::make(42.0));
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_NUMBER_I2);
        assert_eq!(i16::from_be_bytes([bytes[1], bytes[2]]), 42);
    }

    #[test]
    fn test_number_i4() {
        let v = Value::from(Number::make(100_000.0));
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_NUMBER_I4);
        assert_eq!(
            i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]),
            100_000
        );
    }

    #[test]
    fn test_number_f8() {
        let v = Value::from(Number::make(3.14));
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_NUMBER_F8);
        let f = f64::from_be_bytes(bytes[1..9].try_into().unwrap());
        assert!((f - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_number_with_unit() {
        let v = Value::from(Number::make_with_unit(100.0, get_unit_or_default("kW")));
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_NUMBER_I2);
    }

    #[test]
    fn test_varint_ranges() {
        let mut buf = Vec::new();
        encode_varint(&mut buf, -1).unwrap();
        assert_eq!(buf, vec![0xff]);

        buf.clear();
        encode_varint(&mut buf, 0).unwrap();
        assert_eq!(buf, vec![0x00]);

        buf.clear();
        encode_varint(&mut buf, 0x7f).unwrap();
        assert_eq!(buf, vec![0x7f]);

        buf.clear();
        encode_varint(&mut buf, 0x80).unwrap();
        assert_eq!(buf, vec![0x80, 0x80]);

        buf.clear();
        encode_varint(&mut buf, 0x3fff).unwrap();
        assert_eq!(buf, vec![0xbf, 0xff]);

        buf.clear();
        encode_varint(&mut buf, 0x4000).unwrap();
        assert_eq!(buf, vec![0xc0, 0x00, 0x40, 0x00]);
    }

    #[test]
    fn test_str_inline_unknown() {
        // "hello" is not in the constants table → inline encoding
        let v = Value::from("hello");
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_STR);
        // inline sentinel 0xff, then varint(5), then "hello"
        assert_eq!(bytes[1], 0xff);
        assert_eq!(bytes[2], 5);
        assert_eq!(&bytes[3..], b"hello");
    }

    #[test]
    fn test_str_const_encoding() {
        // "dis" is const index 185; varint(185) = [0x80, 0xb9]
        let v = Value::from("dis");
        let bytes = enc(&v);
        assert_eq!(bytes, vec![CTRL_STR, 0x80, 0xb9]);
    }

    #[test]
    fn test_dict_key_uses_const() {
        // Keys like "dis" and "site" should be encoded as const indices.
        // We verify this by checking the overall bytes are shorter than inline
        // would produce (3 vs 1+1+1+3 = 6 bytes per key).
        let dict_val = Value::make_dict(dict! { "dis" => Value::make_marker() });
        let bytes_with_consts = enc(&dict_val);
        // Build what inline-only would look like for key "dis" (len 3):
        // 0xff 0x03 'd' 'i' 's' = 5 bytes vs 2 bytes with const
        // Total with consts: CTRL_DICT + '{' + count(1) + key(2) + CTRL_MARKER + '}'
        //                  = 1 + 1 + 1 + 2 + 1 + 1 = 7
        // Total inline:      1 + 1 + 1 + 5 + 1 + 1 = 10
        assert!(
            bytes_with_consts.len() < 10,
            "const encoding should be shorter than inline"
        );
    }

    #[test]
    fn test_unit_uses_const() {
        // "kW" is const index 630; the unit string in the number should be compact.
        use crate::haystack::encoding::brio::consts::lookup_const;
        assert_eq!(lookup_const("kW"), Some(630));
        let v = Value::from(Number::make_with_unit(50.0, get_unit_or_default("kW")));
        let bytes = enc(&v);
        // byte 0 = CTRL_NUMBER_I2, bytes 1-2 = i16(50)
        // then unit string as varint(630): 0x8000 | 630 = 0x8276 → [0x82, 0x76]
        assert_eq!(bytes[3], 0x82);
        assert_eq!(bytes[4], 0x76);
    }

    #[test]
    fn test_timezone_uses_const() {
        // The "UTC" timezone should be encoded as const index 24.
        let dt = DateTime::parse_from_rfc3339("2021-06-15T12:00:00Z").unwrap();
        let bytes = enc(&Value::from(dt));
        // CTRL_DATETIME_I4 + 4 bytes + varint(24) = 1+4+1 = 6
        assert_eq!(bytes.len(), 6);
        assert_eq!(bytes[0], CTRL_DATETIME_I4);
        assert_eq!(bytes[5], 24); // varint(24) = single byte 0x18
    }

    #[test]
    fn test_str_cesu8_null_char() {
        // U+0000 must be encoded as 0xC0 0x80 (Modified UTF-8), not a bare 0x00,
        // matching Fantom/JVM OutStream.writeChar behaviour.
        let v = Value::from("\0");
        let bytes = enc(&v);
        // CTRL_STR + varint(-1) + varint(1) + 0xC0 0x80
        assert_eq!(bytes, vec![CTRL_STR, 0xff, 0x01, 0xC0, 0x80]);
    }

    #[test]
    fn test_str_cesu8_supplementary_char() {
        // U+1F600 (😀) has UTF-16 surrogate pair [0xD83D, 0xDE00].
        // Each surrogate is encoded as a 3-byte CESU-8 sequence —
        // 6 bytes total, matching what Fantom/JVM produces.
        let v = Value::from("\u{1F600}");
        let bytes = enc(&v);
        // CTRL_STR + varint(-1) + varint(2) + CESU-8(0xD83D) + CESU-8(0xDE00)
        assert_eq!(
            bytes,
            vec![
                CTRL_STR, 0xff, // inline sentinel
                0x02, // 2 UTF-16 code units
                0xED, 0xA0, 0xBD, // high surrogate 0xD83D
                0xED, 0xB8, 0x80, // low  surrogate 0xDE00
            ]
        );
    }

    #[test]
    fn test_str_cesu8_round_trip() {
        // Verify that a string with a null char, BMP chars, and a supplementary
        // char all survive a full encode → decode round-trip.
        let original = Value::from("a\0b\u{1F600}c");
        let bytes = enc(&original);
        let decoded = crate::encoding::brio::decode::from_brio(&mut bytes.as_slice()).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_uri() {
        let v = Value::from(Uri {
            value: "http://example.com".into(),
        });
        let bytes = enc(&v);
        assert_eq!(bytes[0], CTRL_URI);
    }

    #[test]
    fn test_ref_str() {
        let r = Ref::make("abc123", None);
        let bytes = enc(&Value::from(r));
        assert_eq!(bytes[0], CTRL_REF_STR);
    }

    #[test]
    fn test_ref_i8() {
        // "cafebabe-deadbeef" packs to 0xcafebabedeadbeef which is negative as i64,
        // so we fall back to CTRL_REF_STR — matching Fantom behaviour.
        let r = Ref::make("cafebabe-deadbeef", None);
        let bytes = enc(&Value::from(r));
        assert_eq!(bytes[0], CTRL_REF_STR);
    }

    #[test]
    fn test_date() {
        let d = Date::from_ymd(2021, 6, 15).unwrap();
        let bytes = enc(&Value::from(d));
        assert_eq!(bytes[0], CTRL_DATE);
        assert_eq!(i16::from_be_bytes([bytes[1], bytes[2]]), 2021);
        assert_eq!(bytes[3], 6);
        assert_eq!(bytes[4], 15);
    }

    #[test]
    fn test_time() {
        let t = Time::from_hms(10, 30, 0).unwrap();
        let bytes = enc(&Value::from(t));
        assert_eq!(bytes[0], CTRL_TIME);
        let millis = i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        assert_eq!(millis, (10 * 3600 + 30 * 60) * 1000);
    }

    #[test]
    fn test_datetime_i4() {
        let dt = DateTime::parse_from_rfc3339("2021-06-15T12:00:00Z").unwrap();
        let bytes = enc(&Value::from(dt));
        assert_eq!(bytes[0], CTRL_DATETIME_I4);
    }

    #[test]
    fn test_coord() {
        let c = Coord::make(45.0, 23.0);
        let bytes = enc(&Value::from(c));
        assert_eq!(bytes[0], CTRL_COORD);
        let lat = i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let lng = i32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        // Fantom packs as (lat+90)*1e6 and (lng+180)*1e6
        assert_eq!(lat, 135_000_000); // (45+90)*1e6
        assert_eq!(lng, 203_000_000); // (23+180)*1e6
    }

    #[test]
    fn test_xstr() {
        let x = XStr::make("Blob", "data");
        let bytes = enc(&Value::from(x));
        assert_eq!(bytes[0], CTRL_XSTR);
    }

    #[test]
    fn test_symbol() {
        let s = Symbol::make("foo");
        let bytes = enc(&Value::from(s));
        assert_eq!(bytes[0], CTRL_SYMBOL);
    }

    #[test]
    fn test_empty_dict() {
        let d = Dict::default();
        let bytes = enc(&Value::from(d));
        assert_eq!(bytes, vec![CTRL_DICT_EMPTY]);
    }

    #[test]
    fn test_non_empty_dict() {
        let d = dict! { "marker" => Value::make_marker() };
        let bytes = enc(&Value::make_dict(d));
        assert_eq!(bytes[0], CTRL_DICT);
        assert_eq!(bytes[1], b'{');
    }

    #[test]
    fn test_empty_list() {
        let l: List = vec![];
        let bytes = enc(&Value::from(l));
        assert_eq!(bytes, vec![CTRL_LIST_EMPTY]);
    }

    #[test]
    fn test_non_empty_list() {
        let l: List = vec![Value::make_marker(), Value::from(true)];
        let bytes = enc(&Value::from(l));
        assert_eq!(bytes[0], CTRL_LIST);
        assert_eq!(bytes[1], b'[');
    }

    #[test]
    fn test_grid() {
        let g = Grid::make_from_dicts(vec![dict! { "dis" => Value::from("Site") }]);
        let bytes = enc(&Value::from(g));
        assert_eq!(bytes[0], CTRL_GRID);
        assert_eq!(bytes[1], b'<');
    }

    // -----------------------------------------------------------------------
    // Haxall interop: encoded byte sizes verified against BrioTest.fan
    //
    // Each assertion references the matching `verifyIO(val, expectedSize)` call
    // in src/test/testHaystack/fan/BrioTest.fan of the Haxall repository.
    // -----------------------------------------------------------------------

    fn enc_size(v: &Value) -> usize {
        enc(v).len()
    }

    /// Measure the encoded byte length of a single varint value.
    fn varint_size(val: i64) -> usize {
        let mut buf: Vec<u8> = Vec::new();
        encode_varint(&mut buf, val).unwrap();
        buf.len()
    }

    /// Leak a custom `Unit` whose only identifier / symbol is `symbol`.
    /// Used only in tests that need a non-database unit (e.g. "_foo").
    fn make_custom_unit(symbol: &'static str) -> &'static Unit {
        Box::leak(Box::new(Unit {
            quantity: None,
            ids: vec![symbol.to_string()],
            dimensions: None,
            scale: 1.0,
            offset: 0.0,
        }))
    }

    #[test]
    fn test_haxall_compat_scalar_sizes() {
        // verifyIO(null, 1)  verifyIO(Marker.val, 1)  etc.
        assert_eq!(enc_size(&Value::Null), 1);
        assert_eq!(enc_size(&Value::make_marker()), 1);
        assert_eq!(enc_size(&Value::make_na()), 1);
        assert_eq!(enc_size(&Value::make_remove()), 1);
        assert_eq!(enc_size(&Value::from(true)), 1);
        assert_eq!(enc_size(&Value::from(false)), 1);
    }

    #[test]
    fn test_haxall_compat_number_sizes() {
        // verifyIO(n(12), 4)
        assert_eq!(enc_size(&Value::from(Number::make(12.0))), 4);
        // verifyIO(n(123_456_789), 6)
        assert_eq!(enc_size(&Value::from(Number::make(123_456_789.0))), 6);
        // verifyIO(n(123_456_789, "°F"), 7)  — unit is const index 730 → 2-byte varint
        assert_eq!(
            enc_size(&Value::from(Number::make_with_unit(
                123_456_789.0,
                get_unit_or_default("°F")
            ))),
            7
        );
        // verifyIO(n(123_456.789f, "°F"), 11) — float + 2-byte unit varint
        assert_eq!(
            enc_size(&Value::from(Number::make_with_unit(
                123_456.789,
                get_unit_or_default("°F")
            ))),
            11
        );
        // verifyIO(n(0x7fff), 4)  verifyIO(n(0x7fff+1), 6)
        assert_eq!(enc_size(&Value::from(Number::make(32767.0))), 4);
        assert_eq!(enc_size(&Value::from(Number::make(32768.0))), 6);
        // verifyIO(n(-32767), 4)  verifyIO(n(-32768), 6)
        // Key Haxall rule: I2 range is -32767..=32767, excluding i16::MIN (-32768)
        assert_eq!(enc_size(&Value::from(Number::make(-32767.0))), 4);
        assert_eq!(enc_size(&Value::from(Number::make(-32768.0))), 6);
        // verifyIO(n(0x7fff_ffff), 6)  verifyIO(n(0x8000_0000), 10)
        assert_eq!(enc_size(&Value::from(Number::make(2_147_483_647.0))), 6);
        assert_eq!(enc_size(&Value::from(Number::make(2_147_483_648.0))), 10);
        // verifyIO(n(-2147483648), 6)  verifyIO(n(-2147483649), 10)
        assert_eq!(enc_size(&Value::from(Number::make(-2_147_483_648.0))), 6);
        assert_eq!(enc_size(&Value::from(Number::make(-2_147_483_649.0))), 10);
    }

    #[test]
    fn test_haxall_compat_string_sizes() {
        // verifyIO("", 2)  — "" is const index 0 → varint(0) = 1 byte
        assert_eq!(enc_size(&Value::from("")), 2);
        // verifyIO("hello °F world!", 3+16)
        // Header: CTRL_STR + 0xff + varint(15) = 3 bytes; CESU-8 body = 16 bytes
        // "°" (U+00B0) is 2 CESU-8 bytes; all others ASCII = 14 bytes → total 16
        assert_eq!(enc_size(&Value::from("hello °F world!")), 19);
        // verifyIO("siteRef", 3)  — "siteRef" is in the const table, 2-byte index
        assert_eq!(enc_size(&Value::from("siteRef")), 3);
        // verifyIO("New_York", 2)  — const index 26, varint = 1 byte
        assert_eq!(enc_size(&Value::from("New_York")), 2);
    }

    #[test]
    fn test_haxall_compat_ref_sizes() {
        // verifyIO(Ref("1deb31b8-7508b187"), 10)
        // CTRL_REF_I8(1) + i64(8) + encodeStrChars("")=varint(0)(1) = 10
        assert_eq!(
            enc_size(&Value::from(Ref::make("1deb31b8-7508b187", None))),
            10
        );
        // verifyIO(Ref("1debX1b8-7508b187"), 21)  — 'X' is not hex → STR form
        // CTRL_REF_STR(1) + inline("1debX1b8-7508b187"): 0xff+varint(17)+17 = 19 + varint(0) = 21
        assert_eq!(
            enc_size(&Value::from(Ref::make("1debX1b8-7508b187", None))),
            21
        );
        // verifyIO(Ref("1deb31b8.7508b187"), 21)  — '.' at pos 8, not '-' → STR form
        assert_eq!(
            enc_size(&Value::from(Ref::make("1deb31b8.7508b187", None))),
            21
        );
        // verifyIO(Ref("1deb31b8-7508b187", "hi!"), 13)
        // CTRL_REF_I8(1) + i64(8) + encodeStrChars("hi!")=varint(3)(1)+"hi!"(3) = 13
        assert_eq!(
            enc_size(&Value::from(Ref::make("1deb31b8-7508b187", Some("hi!")))),
            13
        );
    }

    #[test]
    fn test_haxall_compat_symbol_sizes() {
        // verifyIO(Symbol("coolingTower"), 3)  — in const table, 2-byte index
        assert_eq!(enc_size(&Value::from(Symbol::make("coolingTower"))), 3);
        // verifyIO(Symbol("foo-bar"), 3+7)
        // CTRL_SYMBOL(1) + 0xff(1) + varint(7)(1) + "foo-bar"(7) = 10
        assert_eq!(enc_size(&Value::from(Symbol::make("foo-bar"))), 10);
    }

    #[test]
    fn test_haxall_compat_container_sizes() {
        // verifyIO(Etc.dict0, 1)
        assert_eq!(enc_size(&Value::from(Dict::default())), 1);
        // verifyIO(Obj?[,], 1)  — empty list
        assert_eq!(enc_size(&Value::from(vec![] as List)), 1);
        // verifyIO(Coord(37.54f, 77.43f), 9)
        assert_eq!(enc_size(&Value::from(Coord::make(37.54, 77.43))), 9);
    }

    #[test]
    fn test_haxall_compat_varint_sizes() {
        // BrioTest.fan testVarInt boundary assertions:
        // vals  := [-1, 0, 30, 64, 127, 128, 1_000, 16_383, 16_384, 500_123, 536_870_911, 536_870_912, 123_456_789_123]
        // sizes := [ 1, 1,  1,  1,   1,   2,     2,      2,      4,       4,           4,           9,               9]
        assert_eq!(varint_size(-1), 1);
        assert_eq!(varint_size(0), 1);
        assert_eq!(varint_size(30), 1);
        assert_eq!(varint_size(64), 1);
        assert_eq!(varint_size(127), 1);
        assert_eq!(varint_size(128), 2);
        assert_eq!(varint_size(1_000), 2);
        assert_eq!(varint_size(16_383), 2);
        assert_eq!(varint_size(16_384), 4);
        assert_eq!(varint_size(500_123), 4);
        assert_eq!(varint_size(536_870_911), 4);
        assert_eq!(varint_size(536_870_912), 9);
        assert_eq!(varint_size(123_456_789_123), 9);
    }

    #[test]
    fn test_haxall_compat_number_nonconstunit() {
        // verifyIO(n(123_456_789, "_foo"), 11)
        // CTRL_NUMBER_I4(1) + i32(4) + inline("_foo"): varint(-1)(1)+varint(4)(1)+"_foo"(4) = 11
        assert_eq!(
            enc_size(&Value::from(Number::make_with_unit(
                123_456_789.0,
                make_custom_unit("_foo")
            ))),
            11
        );
    }

    #[test]
    fn test_haxall_compat_coord_negative() {
        // verifyIO(Coord(-17.535f, -149.569f), 9)
        assert_eq!(enc_size(&Value::from(Coord::make(-17.535, -149.569))), 9);
    }

    #[test]
    fn test_haxall_compat_datetime_sizes() {
        fn dt_size(iso: &str, tz: &str) -> usize {
            let dt = DateTime::parse_from_rfc3339_with_timezone(iso, tz).unwrap();
            enc_size(&Value::from(dt))
        }
        // verifyIO(DateTime("2015-11-30T12:03:57-05:00 New_York"), 6)
        // I4 (no sub-seconds), New_York = const 26 → 1-byte varint: CTRL(1)+i32(4)+varint(1) = 6
        assert_eq!(dt_size("2015-11-30T12:03:57-05:00", "New_York"), 6);
        // verifyIO(DateTime("2015-11-30T12:02:33.378-05:00 New_York"), 10)
        // I8 (has millis), New_York const: CTRL(1)+i64(8)+varint(1) = 10
        assert_eq!(dt_size("2015-11-30T12:02:33.378-05:00", "New_York"), 10);
        // verifyIO(DateTime("2015-11-30T12:03:57.000123-05:00 New_York"), 10)
        // I8 (non-zero sub-second): CTRL(1)+i64(8)+varint(1) = 10
        assert_eq!(dt_size("2015-11-30T12:03:57.000123-05:00", "New_York"), 10);
        // verifyIO(DateTime("2000-01-01T00:00:00+01:00 Warsaw"), 13)
        // I4, "Warsaw" not in const → inline(8): CTRL(1)+i32(4)+8 = 13
        assert_eq!(dt_size("2000-01-01T00:00:00+01:00", "Warsaw"), 13);
        // verifyIO(DateTime("2000-01-01T00:00:00.832+01:00 Warsaw"), 17)
        // I8, Warsaw inline(8): CTRL(1)+i64(8)+8 = 17
        assert_eq!(dt_size("2000-01-01T00:00:00.832+01:00", "Warsaw"), 17);
        // verifyIO(DateTime("1999-06-07T01:02:00-04:00 New_York"), 6)
        assert_eq!(dt_size("1999-06-07T01:02:00-04:00", "New_York"), 6);
        // verifyIO(DateTime("1950-06-07T01:02:00-04:00 New_York"), 6)
        assert_eq!(dt_size("1950-06-07T01:02:00-04:00", "New_York"), 6);
        // verifyIO(DateTime("1950-06-07T01:02:00.123-04:00 New_York"), 10)
        assert_eq!(dt_size("1950-06-07T01:02:00.123-04:00", "New_York"), 10);
    }
}
