// Copyright (C) 2020 - 2022, J2 Innovations

//! Test haystack types

use libhaystack::units::get_unit_or_default;

#[cfg(test)]
use crate::libhaystack::dict;
use crate::libhaystack::val::*;

#[test]
fn test_value_null() {
    let value = Value::default();
    assert!(value.is_null());
    assert!(!value.is_number());
}

#[test]
fn test_value_marker() {
    let value = Value::Marker;
    assert!(value.is_marker());
    assert!(!value.is_bool());
    assert!(value.has_value());

    assert!(Value::make_marker().is_marker());
}

#[test]
fn test_value_na() {
    let value = Value::Na;
    assert!(value.is_na());

    assert!(Value::make_na().is_na());
}

#[test]
fn test_value_remove() {
    let value = Value::Remove;
    assert!(value.is_remove());

    assert!(Value::make_remove().is_remove());
}

#[test]
fn test_value_bool() {
    let mut value = Value::Bool(Bool::from(true));
    assert!(value.is_bool());
    assert_eq!(bool::try_from(&value), Ok(true));

    value = Value::Bool(Bool::from(false));
    assert!(!bool::try_from(&value).unwrap());

    assert!(Value::make_bool(true).is_bool());
    assert_eq!(Value::make_bool(true), Value::Bool(Bool::from(true)));

    assert_eq!(bool::try_from(&Value::make_bool(false)), Ok(false));
    value = true.into();
    assert_eq!(value, Value::make_bool(true));

    let bool_val = bool::try_from(&value);
    assert!(!bool_val.is_err());
    assert!(bool_val.unwrap());

    value = Value::from(true);
    assert!(value.is_bool());
    assert_eq!(bool::try_from(&value), Ok(true));

    assert!(Value::make_true().is_true());
    assert!(!Value::make_false().is_true());

    assert!(Value::make_false().is_false());
    assert!(!Value::make_false().is_true());
}

#[test]
fn test_value_num() {
    let value = Value::Number(Number {
        value: 100.0,
        unit: None,
    });
    assert!(&value.is_number());
    assert_eq!(
        Number::try_from(&value).unwrap(),
        Number {
            value: 100.0,
            unit: None
        }
    );
    assert_eq!(f64::try_from(&value).unwrap(), 100.0);

    assert!(Value::make_number(42.0).is_number());
    assert!(Value::make_number_unit(42.0, get_unit_or_default("m/s")).is_number());

    assert_eq!(
        Number::try_from(&Value::make_number_unit(42.0, get_unit_or_default("°C")))
            .unwrap()
            .value,
        42.0
    );

    assert_eq!(
        Number::try_from(&Value::make_number_unit(42.0, get_unit_or_default("°C")))
            .unwrap()
            .unit,
        Some(get_unit_or_default("°C"))
    );

    assert_eq!(Value::from(100), Value::make_number(100.00));
}

#[test]
fn test_value_str() {
    let value = Value::Str(Str {
        value: String::from("Foo"),
    });
    assert!(value.is_str());
    assert_eq!(Value::from("Foo"), value);

    assert_eq!(Str::try_from(&value).unwrap(), Str::from("Foo"));

    assert_eq!(String::try_from(&value), Ok(String::from("Foo")));

    assert_eq!(Value::make_str("Foo"), value);
}

#[test]
fn test_value_ref() {
    let ref_ = Ref {
        value: String::from("someId"),
        dis: Some(String::from("dis")),
    };

    let value = Value::Ref(ref_.clone());
    assert!(value.is_ref());
    assert_eq!(&Ref::try_from(&value).unwrap(), &ref_);
    assert_eq!(&Ref::try_from(&value).unwrap().dis.unwrap(), "dis");

    assert_eq!(Value::from(Ref::from("someId")), Value::make_ref("someId"));
}

#[test]
fn test_value_symbol() {
    let value = Value::make_symbol("foo");
    assert!(value.is_symbol());
    assert_eq!(&Symbol::try_from(&value).unwrap(), &Symbol::from("foo"));
    assert_eq!(Symbol::try_from(&value).unwrap().value, "foo");
    assert_eq!(
        Value::from(Symbol::from("symbol")),
        Value::make_symbol("symbol")
    );

    let value = Value::Symbol(Symbol::make("abc"));
    assert_eq!(Symbol::try_from(&value), Ok(Symbol::make("abc")));
}

#[test]
fn test_value_uri() {
    let value = Value::make_uri("http://zoo.bar");
    assert!(value.is_uri());
    assert_eq!(
        &Uri::try_from(&value).unwrap(),
        &Uri::from("http://zoo.bar")
    );
    assert_eq!(Uri::try_from(&value).unwrap().value, "http://zoo.bar");
    assert_eq!(Value::from(Uri::from("uri")), Value::make_uri("uri"));

    let value = Value::Uri(Uri::make("/a/b/c"));
    assert_eq!(Uri::try_from(&value), Ok(Uri::make("/a/b/c")));
}

#[test]
fn test_value_date() {
    let value = Value::make_date(Date::from_ymd(2020, 3, 22).expect("Date"));
    assert!(value.is_date());
    assert_eq!(
        &Date::try_from(&value).expect("Date Value"),
        &Date::from_ymd(2020, 3, 22).expect("Date")
    );

    let value = Value::Date(Date::from_ymd(2020, 3, 22).expect("Date"));
    assert_eq!(
        Date::try_from(&value),
        Ok(Date::from_ymd(2020, 3, 22).expect("Date"))
    );
}

#[test]
fn test_value_time() {
    let value = Value::make_time(Time::from_hms(17, 25, 33).expect("Time"));
    assert!(value.is_time());
    assert_eq!(
        &Time::try_from(&value).unwrap(),
        &Time::from_hms(17, 25, 33).expect("Time")
    );

    let value = Value::Time(Time::from_hms(17, 25, 33).expect("Time"));
    assert_eq!(
        Time::try_from(&value),
        Ok(Time::from_hms(17, 25, 33).expect("Time"))
    );
}

#[test]
fn test_value_datetime() {
    let value = Value::make_datetime_from_iso("1996-12-19T16:39:57-08:00").unwrap();
    assert!(value.is_datetime());
    assert_eq!(
        &DateTime::try_from(&value).unwrap(),
        &DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap()
    );

    let value = Value::DateTime(
        DateTime::parse_from_rfc3339("2021-06-20T19:48:23-00:00").expect("DateTime"),
    );
    assert_eq!(
        DateTime::try_from(&value),
        Ok(DateTime::parse_from_rfc3339("2021-06-20T19:48:23-00:00").expect("DateTime"))
    );
}

#[test]
fn test_value_coord() {
    let value = Value::make_coord(Coord {
        lat: 45.0,
        long: 23.0,
    });
    assert!(value.is_coord());

    assert_eq!(value, Value::make_coord_from(45.0, 23.0));

    assert_eq!(&Coord::try_from(&value).unwrap(), &Coord::make(45.0, 23.0));

    let value = Value::Coord(Coord {
        lat: 45.0,
        long: 23.0,
    });
    assert!(value.is_coord());
}

#[test]
fn test_value_xstr() {
    let value = Value::make_xstr_from("Foo", "bar");
    assert!(value.is_xstr());

    assert_eq!(value, Value::make_xstr(XStr::make("Foo", "bar")));
}

#[test]
fn test_value_list() {
    let value = Value::make_list(vec![
        Value::make_marker(),
        Value::make_bool(true),
        Value::make_str("foo"),
        Value::make_list(vec![Value::make_number(42.0)]),
    ]);
    assert!(value.is_list());

    assert_eq!(List::try_from(&value).unwrap()[0], Value::make_marker());
    assert_eq!(List::try_from(&value).unwrap()[1], Value::make_bool(true));
    assert_eq!(List::try_from(&value).unwrap()[2], Value::make_str("foo"));
    assert_eq!(
        List::try_from(&value).unwrap()[3],
        Value::make_list(vec![Value::make_number(42.0)])
    );
}

#[test]
fn test_value_dict() {
    let rec = dict! {
        "site" => Value::make_marker(),
        "name" => Value::make_str("Foo")
    };
    let value = Value::make_dict(rec);
    assert!(value.is_dict());

    assert_eq!(
        Dict::try_from(&value).unwrap().get(&String::from("site")),
        Some(&Value::make_marker())
    )
}

#[test]
fn test_value_dict_value() {
    let dict = Value::from(dict! {
        "site" => Value::make_marker(),
        "name" => Value::from("Foo")
    });
    assert!(dict.is_dict());

    // Get the Dict value
    let dict_value = Dict::try_from(&dict).unwrap();
    assert!(dict_value.has("site"));

    let dict = Value::Dict(dict! {
        "site" => Value::make_marker(),
        "name" => Value::from("Foo")
    });
    assert!(dict.is_dict());
}

#[test]
fn test_value_grid() {
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
    let value = Value::make_grid(Grid::make_from_dicts(recs));
    assert!(value.is_grid());

    let grid = Grid::try_from(&value).unwrap();
    assert!(!grid.is_empty());
    assert!(!grid.columns.is_empty());
    assert_eq!(
        grid[1],
        dict! {
            "equip" => Value::make_marker(),
            "navName" => Value::make_str("Equip")
        }
    );

    let mut cols = (&grid.columns)
        .iter()
        .map(|col| col.name.clone())
        .collect::<Vec<String>>();
    cols.sort();

    assert_eq!(
        cols,
        vec![
            String::from("dis"),
            String::from("equip"),
            String::from("navName"),
            String::from("site")
        ]
    );

    let value = Value::Grid(Grid::make_empty());
    assert!(value.is_grid());
}
