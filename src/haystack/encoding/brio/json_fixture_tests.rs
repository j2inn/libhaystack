// Copyright (C) 2020 - 2022, J2 Innovations

//! Brio encoding/decoding tests derived from real-world JSON haystack data.
//!
//! The fixture data was captured from a system where Brio encoding/decoding failed,
//! exposing the bug that `Dict` encodes `null`-valued tags into the Brio stream
//! instead of silently skipping them — which is what the Haxall reference
//! implementation (`BrioWriter.fan`) does.
//!
//! ## The Bug
//!
//! `BrioWriter.fan` (Haxall reference) explicitly skips null-valued dict entries:
//!
//! ```fantom
//! count := 0
//! dict.each |val, name| { if (val != null) count++ }  // nulls excluded from count
//! dict.each |val, name| { if (val == null) return; ... }  // nulls not written
//! ```
//!
//! The previous libhaystack implementation used `self.len()` (including nulls) and
//! wrote a `CTRL_NULL` byte for every null-valued tag, producing bytes incompatible
//! with any Haxall consumer.
//!
//! The JSON data contained numerous null-valued dict entries, e.g.:
//! - `priorityArray` items: `{"level":1,"val":null}`
//! - `{"duration":null,"level":15,"val":{"_kind":"number",...},"who":null}`
//! - Top-level: `{"originRef":null, "id":...}`
//!
//! Tests named `*_null_*skipped*` or `*_haxall_compat*` are the **primary bug tests**.

#[cfg(all(test, feature = "brio-encoding", feature = "brio-decoding"))]
mod tests {
    use crate::dict;
    use crate::encoding::brio::decode::from_brio;
    use crate::encoding::brio::encode::ToBrio;
    use crate::haystack::val::{Bool, DateTime, Dict, List, Marker, Number, Ref, Value};
    use crate::units::get_unit_or_default;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn round_trip(v: &Value) -> Value {
        let bytes = v.to_brio_vec().expect("encode");
        from_brio(&mut bytes.as_slice()).expect("decode")
    }

    // -----------------------------------------------------------------------
    // THE BUG TESTS
    // These assertions directly verify the Haxall-specified behaviour that
    // null-valued dict entries MUST be omitted from the Brio byte stream.
    // -----------------------------------------------------------------------

    /// A dict with a null-valued tag must encode identically to the same dict
    /// without that tag.  This is the canonical regression test for the bug.
    #[test]
    fn test_haxall_compat_dict_null_skipped() {
        let with_null = Value::make_dict(dict! {
            "level" => Value::from(Number::make(1.0)),
            "val"   => Value::Null
        });
        let without_null = Value::make_dict(dict! {
            "level" => Value::from(Number::make(1.0))
        });
        assert_eq!(
            with_null.to_brio_vec().unwrap(),
            without_null.to_brio_vec().unwrap(),
            "null-valued dict entries must be silently skipped (BrioWriter.fan spec)"
        );
    }

    /// Dicts where ALL present tags are null should encode as CTRL_DICT_EMPTY (0x14),
    /// not as a CTRL_DICT with null entries.
    #[test]
    fn test_all_null_dict_becomes_empty() {
        let all_null = Value::make_dict(dict! {
            "val" => Value::Null
        });
        let empty = Value::from(Dict::default());
        assert_eq!(
            all_null.to_brio_vec().unwrap(),
            empty.to_brio_vec().unwrap(),
            "a dict whose only tags are null must encode as CTRL_DICT_EMPTY"
        );
    }

    /// Multiple null tags in a single dict must all be skipped independently.
    /// This mirrors the `{"duration":null,"level":15,"val":Number,"who":null}` pattern
    /// from the priority-array items in the captured JSON data.
    #[test]
    fn test_haxall_compat_dict_multiple_nulls_skipped() {
        let unit_c = get_unit_or_default("°C");

        let with_nulls = Value::make_dict(dict! {
            "duration" => Value::Null,
            "level"    => Value::from(Number::make(15.0)),
            "val"      => Value::from(Number::make_with_unit(19.002_716_819_775_618, unit_c)),
            "who"      => Value::Null
        });
        let without_nulls = Value::make_dict(dict! {
            "level" => Value::from(Number::make(15.0)),
            "val"   => Value::from(Number::make_with_unit(19.002_716_819_775_618, unit_c))
        });
        assert_eq!(
            with_nulls.to_brio_vec().unwrap(),
            without_nulls.to_brio_vec().unwrap(),
            "multiple null-valued dict tags must all be skipped"
        );
    }

    /// A top-level dict tag whose value is null must not appear in the encoded output.
    /// This mirrors the `{"originRef":null, "id":Ref(...)}` pattern in the last
    /// record of the captured JSON data.
    #[test]
    fn test_dict_with_null_top_level_tag_skipped() {
        let with_null = Value::make_dict(dict! {
            "id"        => Value::from(Ref::make("315672b6-46c209e2", Some("315672b6-46c209e2"))),
            "originRef" => Value::Null
        });
        let without_null = Value::make_dict(dict! {
            "id" => Value::from(Ref::make("315672b6-46c209e2", Some("315672b6-46c209e2")))
        });
        assert_eq!(
            with_null.to_brio_vec().unwrap(),
            without_null.to_brio_vec().unwrap(),
            "null originRef must be silently skipped"
        );
    }

    // -----------------------------------------------------------------------
    // Round-trip tests for data types present in the fixture JSON
    // -----------------------------------------------------------------------

    /// Both I2 (23.0) and F8 (19.002716…) °C number encodings must round-trip.
    #[test]
    fn test_number_with_deg_c_unit_round_trip() {
        let unit_c = get_unit_or_default("°C");

        let v_i2 = Value::from(Number::make_with_unit(23.0, unit_c));
        assert_eq!(round_trip(&v_i2), v_i2, "23 °C (I2 path) round-trip failed");

        let v_f8 = Value::from(Number::make_with_unit(19.002_716_819_775_618, unit_c));
        assert_eq!(
            round_trip(&v_f8),
            v_f8,
            "19.0027… °C (F8 path) round-trip failed"
        );
    }

    /// The 17-char hex ref format `xxxxxxxx-xxxxxxxx` from the fixture records
    /// must pack to RefI8; all leading bits must be zero for the pack to succeed.
    #[test]
    fn test_ref_i8_format_315672b6() {
        // id "315672b6-8e6d743a" → 0x315672b68e6d743a — MSB clear, uses RefI8
        let no_dis = Value::from(Ref::make("315672b6-8e6d743a", None));
        assert_eq!(
            round_trip(&no_dis),
            no_dis,
            "RefI8 no-dis round-trip failed"
        );

        let with_dis = Value::from(Ref::make("315672b6-8e6d743a", Some("315672b6-8e6d743a")));
        assert_eq!(
            round_trip(&with_dis),
            with_dis,
            "RefI8 with-dis round-trip failed"
        );

        // Verify each of the ref IDs from the fixture data
        for id in &[
            "315672b6-a5fbc8a5",
            "315672b6-b7160af0",
            "315672b6-f7ad79af",
            "315672b6-ff4b45a2",
            "315672b6-7f3f0944",
            "315672b6-46c209e2",
            "315672b6-49e94ba6",
        ] {
            let v = Value::from(Ref::make(id, None));
            assert_eq!(round_trip(&v), v, "round-trip failed for ref {id}");
        }
    }

    /// DateTimes from the fixture have microsecond sub-second precision, which
    /// selects the `CTRL_DATETIME_I8` (nanosecond) encoding path.
    #[test]
    fn test_datetime_microsecond_precision() {
        // "2025-02-21T14:49:17.261337Z" — 261337 µs → 261337000 ns (non-zero nanos → I8)
        let dt = DateTime::parse_from_rfc3339_with_timezone("2025-02-21T14:49:17.261337Z", "UTC")
            .expect("parse dt");
        let v = Value::from(dt);
        let decoded = round_trip(&v);
        let decoded_dt = DateTime::try_from(&decoded).expect("expected DateTime");
        assert_eq!(
            decoded_dt.timestamp(),
            dt.timestamp(),
            "seconds portion mismatch"
        );
        assert_eq!(
            decoded_dt.timestamp_subsec_nanos(),
            dt.timestamp_subsec_nanos(),
            "sub-second nanos mismatch"
        );

        // "2025-02-21T12:54:50.052481Z" — second fixture datetime
        let dt2 = DateTime::parse_from_rfc3339_with_timezone("2025-02-21T12:54:50.052481Z", "UTC")
            .expect("parse dt2");
        let v2 = Value::from(dt2);
        let decoded2 = round_trip(&v2);
        let decoded_dt2 = DateTime::try_from(&decoded2).expect("expected DateTime");
        assert_eq!(
            decoded_dt2.timestamp_subsec_nanos(),
            dt2.timestamp_subsec_nanos()
        );
    }

    /// The `priorityArray` list contains 17 dict items, most with null `val`.
    /// After round-trip the null-valued entries must be absent from each decoded dict.
    #[test]
    fn test_null_levels_in_priority_array_round_trip() {
        let unit_c = get_unit_or_default("°C");

        // Build a representative slice of the priorityArray:
        //   level 1  → {"level":1, "val":null}
        //   level 15 → {"duration":null, "level":15, "val":Number(19.002…°C), "who":null}
        //   level 17 → {"duration":null, "level":17, "val":Number(23.0°C), "who":"su"}
        let level1 = Value::make_dict(dict! {
            "level" => Value::from(Number::make(1.0)),
            "val"   => Value::Null
        });
        let level15 = Value::make_dict(dict! {
            "duration" => Value::Null,
            "level"    => Value::from(Number::make(15.0)),
            "val"      => Value::from(Number::make_with_unit(19.002_716_819_775_618, unit_c)),
            "who"      => Value::Null
        });
        let level17 = Value::make_dict(dict! {
            "duration" => Value::Null,
            "level"    => Value::from(Number::make(17.0)),
            "val"      => Value::from(Number::make_with_unit(23.0, unit_c)),
            "who"      => Value::from("su")
        });

        let list: Value = Value::from(vec![level1, level15, level17]);
        let bytes = list.to_brio_vec().expect("encode list");
        let decoded = from_brio(&mut bytes.as_slice()).expect("decode list");
        let decoded_list = List::try_from(&decoded).expect("expected List");

        assert_eq!(decoded_list.len(), 3);

        // level 1: null val must be absent
        let d1 = Dict::try_from(&decoded_list[0]).expect("dict 0");
        assert!(
            !d1.contains_key("val"),
            "null 'val' must not appear in decoded dict"
        );
        assert_eq!(
            Number::try_from(d1.get("level").unwrap()).unwrap().value,
            1.0
        );

        // level 15: only non-null tags survive
        let d15 = Dict::try_from(&decoded_list[1]).expect("dict 1");
        assert!(
            !d15.contains_key("duration"),
            "null 'duration' must not appear"
        );
        assert!(!d15.contains_key("who"), "null 'who' must not appear");
        assert!(d15.get("val").is_some(), "'val' must survive (non-null)");

        // level 17: only 'who'="su" survives among the nullable tags
        let d17 = Dict::try_from(&decoded_list[2]).expect("dict 2");
        assert!(
            !d17.contains_key("duration"),
            "null 'duration' must not appear"
        );
        assert!(d17.get("who").is_some(), "'who'=su must survive");
        assert_eq!(d17.get("who").unwrap(), &Value::from("su"));
    }

    /// A representative subset of the captured thermostat setpoint record tags,
    /// enough to exercise markers, refs, strings, a microsecond datetime, and a
    /// number-with-unit together in a single dict round-trip.
    #[test]
    fn test_thermostat_setpoint_record() {
        let unit_c = get_unit_or_default("°C");
        let mod_dt =
            DateTime::parse_from_rfc3339_with_timezone("2025-02-21T14:49:17.261337Z", "UTC")
                .expect("parse mod");

        let record = Value::make_dict(dict! {
            "cur"             => Value::from(Marker),
            "curTracksWrite"  => Value::from(Marker),
            "disMacro"        => Value::from("$equipRef $navName"),
            "equipRef"        => Value::from(Ref::make("315672b6-a5fbc8a5", None)),
            "his"             => Value::from(Marker),
            "hisCollectCov"   => Value::from(Marker),
            "id"              => Value::from(Ref::make("315672b6-8e6d743a", Some("315672b6-8e6d743a"))),
            "kind"            => Value::from("Number"),
            "mod"             => Value::from(mod_dt),
            "navName"         => Value::from("set point"),
            "point"           => Value::from(Marker),
            "siteRef"         => Value::from(Ref::make("315672b6-b7160af0", None)),
            "sp"              => Value::from(Marker),
            "unit"            => Value::from("°C"),
            "writable"        => Value::from(Marker)
        });

        let decoded = round_trip(&record);
        let d = Dict::try_from(&decoded).expect("Dict");

        assert!(d.contains_key("cur"), "cur marker missing");
        assert!(d.contains_key("sp"), "sp marker missing");
        assert_eq!(d.get("navName").unwrap(), &Value::from("set point"));
        assert_eq!(d.get("unit").unwrap(), &Value::from("°C"));

        // Verify the °C number tag from the nested set-point value (standalone)
        let sp_val = Value::from(Number::make_with_unit(23.0, unit_c));
        assert_eq!(round_trip(&sp_val), sp_val);
    }

    /// The `writable` marker, `cmd` marker, and `Bool` true value from the
    /// heating point record (level-15 bool priority item) must round-trip cleanly.
    #[test]
    fn test_bool_true_in_priority_level() {
        // {"duration":null, "level":15, "val":true, "who":null}
        let level_bool = Value::make_dict(dict! {
            "duration" => Value::Null,
            "level"    => Value::from(Number::make(15.0)),
            "val"      => Value::from(Bool::from(true)),
            "who"      => Value::Null
        });

        let decoded = round_trip(&level_bool);
        let d = Dict::try_from(&decoded).expect("Dict");

        assert!(
            !d.contains_key("duration"),
            "null 'duration' must be absent"
        );
        assert!(!d.contains_key("who"), "null 'who' must be absent");
        assert_eq!(d.get("val").unwrap(), &Value::from(true));
    }

    // -----------------------------------------------------------------------
    // GAP FIX TESTS — verifying the three additional Haxall-compatibility fixes
    // -----------------------------------------------------------------------

    /// CTRL_BUF (0x13): a raw binary buffer from Haxall must decode to
    /// `XStr("Bin", "<lowercase hex>")`.  libhaystack has no `Bin` value kind
    /// so we surface it as an XStr to avoid data loss.
    #[test]
    fn test_ctrl_buf_decodes_to_xstr_bin() {
        use crate::encoding::brio::encode::CTRL_BUF;
        use crate::haystack::val::XStr;

        // Build a minimal CTRL_BUF stream: ctrl | varint(3) | 0x01 0x02 0x03
        let mut raw: &[u8] = &[CTRL_BUF, 0x03, 0x01, 0x02, 0x03];
        let decoded = from_brio(&mut raw).expect("decode CTRL_BUF");
        let xs = XStr::try_from(&decoded).expect("XStr");
        assert_eq!(xs.r#type, "Bin");
        assert_eq!(xs.value, "010203");
    }

    /// CTRL_BUF with a zero-length payload should produce `XStr("Bin", "")`.
    #[test]
    fn test_ctrl_buf_empty_decodes_to_empty_xstr_bin() {
        use crate::encoding::brio::encode::CTRL_BUF;
        use crate::haystack::val::XStr;

        let mut raw: &[u8] = &[CTRL_BUF, 0x00];
        let decoded = from_brio(&mut raw).expect("decode empty CTRL_BUF");
        let xs = XStr::try_from(&decoded).expect("XStr");
        assert_eq!(xs.r#type, "Bin");
        assert_eq!(xs.value, "");
    }

    /// Strings with indices > MAX_SAFE_CONST_CODE (945) must encode as inline
    /// strings, not as constant‐table references.  "admin" is at index 946 and
    /// "su" at 964 in the Haxall constant table — both are above the cap.
    #[test]
    fn test_max_safe_const_code_strings_encode_inline() {
        use crate::encoding::brio::consts::{MAX_SAFE_CONST_CODE, lookup_const};

        // Sanity-check that these strings ARE in the table above the cap.
        // (If the table changes this test will tell us.)
        assert!(
            lookup_const("admin").is_none(),
            "'admin' should not be encodable as a const (above MAX_SAFE_CONST_CODE)"
        );
        assert!(
            lookup_const("su").is_none(),
            "'su' should not be encodable as a const (above MAX_SAFE_CONST_CODE)"
        );

        // "ver" is at index 945 — right at the boundary, must still be encodable.
        let ver_idx = lookup_const("ver").expect("'ver' at index 945 should be encodable");
        assert_eq!(
            ver_idx, MAX_SAFE_CONST_CODE,
            "'ver' must map to MAX_SAFE_CONST_CODE"
        );

        // Round-trip: a dict keyed by "admin" must survive encode → decode intact.
        let d = Value::make_dict(dict! { "admin" => Value::from(Marker) });
        let decoded = round_trip(&d);
        let result = Dict::try_from(&decoded).expect("Dict");
        assert!(
            result.contains_key("admin"),
            "'admin' key must survive round-trip"
        );
    }

    /// Numbers whose unit string is not present in the units database should
    /// decode to a unitless Number rather than inheriting DEFAULT_UNIT.
    /// (Requires the `units-db` feature; the test is a no-op otherwise.)
    #[cfg(feature = "units-db")]
    #[test]
    fn test_unknown_unit_decodes_to_unitless() {
        use crate::haystack::val::Number;

        // CTRL_NUMBER_F8 wire format: ctrl(1) | f64-BE(8) | decode_str(unit)
        // decode_str for an inline unit: 0xFF (varint -1) | varint(char_count) | CESU-8 bytes
        // "unknownUnit999" is 14 ASCII chars → char_count = 0x0e
        let value_bytes = 42.0_f64.to_be_bytes();
        let unit_name = b"unknownUnit999";
        let mut raw: Vec<u8> = vec![0x08]; // CTRL_NUMBER_F8
        raw.extend_from_slice(&value_bytes);
        raw.push(0xFF); // varint -1 = inline string
        raw.push(unit_name.len() as u8); // varint char_count
        raw.extend_from_slice(unit_name); // CESU-8 (all ASCII)

        let decoded =
            from_brio(&mut raw.as_slice()).expect("decode CTRL_NUMBER_F8 with unknown unit");
        let n = Number::try_from(&decoded).expect("Number");
        assert_eq!(n.value, 42.0);
        assert!(
            n.unit.is_none(),
            "unknown unit must decode to unitless, got {:?}",
            n.unit
        );
    }
}
