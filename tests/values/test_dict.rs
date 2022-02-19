// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Dict

#[cfg(test)]
use libhaystack::dict;
use libhaystack::filter::Filter;
use libhaystack::filter::Filtered;
use libhaystack::val::*;
use std::iter::FromIterator;
use std::str::FromStr;

#[test]
fn test_dict_make() {
    let dict: Dict = Dict::from_iter([("a".into(), 100.into()), ("b".into(), Marker.into())]);

    assert_eq!(dict.len(), 2);
}

#[test]
fn test_dict_make_value() {
    let dict: Dict = dict!["a" => "a".into(), "b" => 100.into(), "c" => true.into()];

    let value: Value = dict.into();

    assert!(value.is_dict());
    assert!(!value.is_list());

    assert_eq!(
        Dict::try_from(&value),
        Ok(dict!["a" => "a".into(), "b" => 100.into(), "c" => true.into()])
    );
}

#[test]
fn test_dict_accessors() {
    let dict: Dict = dict![
    "a" => "a".into(),
    "b" => 100.into(),
    "c" => true.into(),
    "d" => Remove.into(),
    "e" => List::from_iter([Value::make_str("str")]).into(),
    "f" => dict!{"na" => Na.into()}.into(),
    "g" => Ref::from("ref").into(),
    "h" => Value::make_symbol("symbol"),
    "i" => Value::make_uri("uri"),
    "j" => Value::make_xstr_from("xstr", "val"),
    "k" => Value::Na,
    "l" => Value::Marker,
    "m" => Time::from_str("20:00:00").expect("Time").into(),
    "n" => Date::from_str("2021-06-19").expect("Date").into(),
    "o" => DateTime::from_str("2021-06-19T19:48:23-00:00").expect("DateTime").into(),
    "p" => Coord::make(34.0522, 118.2437).into(),
    "q" => Grid::make_empty().into()
    ];

    assert_eq!(dict.len(), 17);
    assert!(('a'..'r').into_iter().all(|n| dict.has(&n.to_string())));

    assert!(dict.missing("x"));

    assert_eq!(dict.get_str("a"), Some(&Str::from("a")));
    assert_eq!(dict.get_num("b"), Some(&Number::from(100)));
    assert_eq!(dict.get_bool("c"), Some(&Bool::from(true)));
    assert_eq!(dict.get_list("e"), Some(&vec!["str".into()]));
    assert_eq!(dict.get_dict("f"), Some(&dict! {"na" => Na.into()}));
    assert_eq!(dict.get_ref("g"), Some(&Ref::from("ref")));
    assert_eq!(dict.get_symbol("h"), Some(&Symbol::from("symbol")));
    assert_eq!(dict.get_uri("i"), Some(&Uri::from("uri")));
    assert_eq!(dict.get_xstr("j"), Some(&XStr::make("xstr", "val")));
    assert_eq!(
        dict.get_time("m"),
        Some(&Time::from_str("20:00:00").expect("Time"))
    );
    assert_eq!(
        dict.get_date("n"),
        Some(&Date::from_str("2021-06-19").expect("Date"))
    );

    assert_eq!(
        dict.get_date_time("o"),
        Some(&DateTime::from_str("2021-06-19T19:48:23-00:00").expect("DateTime"))
    );
    assert_eq!(dict.get_coord("p"), Some(&Coord::make(34.0522, 118.2437)));
    assert_eq!(dict.get_grid("q"), Some(&Grid::make_empty()));

    assert!(dict.has_remove("d"));
    assert!(dict.has_na("k"));
    assert!(dict.has_marker("l"));

    assert_eq!(dict["a"], Value::make_str("a"));
}

#[test]
fn test_dict_make_filtered() {
    let dict: Dict = dict!["a" => "a".into(), "b" => 100.into(), "c" => true.into()];

    assert!(dict.filter(&Filter::try_from("a and b == 100").unwrap()));
}
