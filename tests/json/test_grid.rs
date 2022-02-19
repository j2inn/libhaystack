// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Grid

#[cfg(test)]
use libhaystack::dict;
use libhaystack::val::*;

#[test]
fn test_json_grid_empty_encode() {
    let string = concat!(
        r#"{"_kind":"grid","meta":{},"#,
        r#""cols":[{"name":"dis"},{"name":"equip"},{"name":"navName"},{"name":"site"}],"#,
        r#""rows":[{"dis":"Site","site":{"_kind":"marker"}},{"equip":{"_kind":"marker"},"navName":"Equip"}]}"#
    );
    let value: Value = serde_json::from_str(string).unwrap();
    let recs = vec![
        dict! {
            "site" => Value::make_marker(),
            "dis" => Value::make_str("Site")
        },
        dict! {
            "equip" => Value::make_marker(),
            "navName" => Value::make_str("Equip")
        },
    ];
    let grid = Value::make_grid(Grid {
        meta: Some(Dict::default()),
        columns: vec![
            Column {
                name: "dis".into(),
                meta: None,
            },
            Column {
                name: "equip".into(),
                meta: None,
            },
            Column {
                name: "navName".into(),
                meta: None,
            },
            Column {
                name: "site".into(),
                meta: None,
            },
        ],
        rows: recs,
    });

    assert_eq!(value, grid);
}

#[test]
fn test_json_grid_decode() {
    let string = concat!(
        r#"{"_kind":"grid","meta":{},"#,
        r#""cols":[{"name":"dis"},{"name":"equip"},{"name":"navName"},{"name":"site"}],"#,
        r#""rows":[{"dis":"Site","site":{"_kind":"marker"}},{"equip":{"_kind":"marker"},"navName":"Equip"}]}"#
    );
    let value: Value = serde_json::from_str(string).unwrap();
    let recs = vec![
        dict! {
            "site" => Value::make_marker(),
            "dis" => Value::make_str("Site")
        },
        dict! {
            "equip" => Value::make_marker(),
            "navName" => Value::make_str("Equip")
        },
    ];
    let grid = Value::make_grid(Grid {
        meta: Some(Dict::default()),
        columns: vec![
            Column {
                name: "dis".into(),
                meta: None,
            },
            Column {
                name: "equip".into(),
                meta: None,
            },
            Column {
                name: "navName".into(),
                meta: None,
            },
            Column {
                name: "site".into(),
                meta: None,
            },
        ],
        rows: recs,
    });

    assert_eq!(value, grid);
}

#[test]
fn test_json_decode_malformed_grid() {
    let string = concat!(
        r#"{"_kind":"grid","meta":{},"#,
        r#""cols":[{"name":"dis"},{"name":"equip"},{"name":"navName"},{"name":"site"}],"#,
        r#""badrows":[{"dis":"Site","site":{"_kind":"marker"}},{"equip":{"_kind":"marker"},"navName":"Equip"}]}"#
    );
    assert!(serde_json::from_str::<Value>(string).is_err());
}
