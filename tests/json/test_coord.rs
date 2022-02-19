// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Coord

#[cfg(test)]
use libhaystack::val::*;

#[test]
fn test_json_coord_encode() {
    let value = Value::make_coord(Coord {
        lat: 45.0,
        long: 23.0,
    });

    let json = serde_json::to_string(&value);
    assert_eq!(json.unwrap(), r#"{"_kind":"coord","lat":45.0,"lng":23.0}"#);
}

#[test]
fn test_json_coord_decode() {
    let value: Value = serde_json::from_str(r#"{"_kind":"coord","lat":45.0,"lng":23.0}"#).unwrap();
    assert_eq!(
        value,
        Value::make_coord(Coord {
            lat: 45.0,
            long: 23.0,
        })
    );
}

#[test]
fn test_json_decode_malformed_coord() {
    assert!(serde_json::from_str::<Value>(r#"{"_kind":"coord","lat":45.0}"#).is_err());
}
