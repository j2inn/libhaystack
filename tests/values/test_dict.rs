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
    let dict: Dict = dict!["a" => "a", "b" => 100, "c" => true];

    let value: Value = dict.into();

    assert!(value.is_dict());
    assert!(!value.is_list());

    assert_eq!(
        Dict::try_from(&value),
        Ok(dict!["a" => "a", "b" => 100, "c" => true])
    );
}

#[test]
fn test_dict_accessors() {
    let dict: Dict = dict![
    "a" => "a",
    "b" => 100,
    "c" => true,
    "d" => Remove,
    "e" => List::from_iter([Value::make_str("str")]),
    "f" => dict!{"na" => Na},
    "g" => Ref::from("ref"),
    "h" => Value::make_symbol("symbol"),
    "i" => Value::make_uri("uri"),
    "j" => Value::make_xstr_from("xstr", "val"),
    "k" => Value::Na,
    "l" => Value::Marker,
    "m" => Time::from_str("20:00:00").expect("Time"),
    "n" => Date::from_str("2021-06-19").expect("Date"),
    "o" => DateTime::from_str("2021-06-19T19:48:23-00:00").expect("DateTime"),
    "p" => Coord::make(34.0522, 118.2437),
    "q" => Grid::make_empty()
    ];

    assert_eq!(dict.len(), 17);
    assert!(('a'..'r').into_iter().all(|n| dict.has(&n.to_string())));

    assert!(dict.missing("x"));

    assert_eq!(dict.get_str("a"), Some(&Str::from("a")));
    assert_eq!(dict.get_num("b"), Some(&Number::from(100)));
    assert_eq!(dict.get_bool("c"), Some(&Bool::from(true)));
    assert_eq!(dict.get_list("e"), Some(&vec!["str".into()]));
    assert_eq!(dict.get_dict("f"), Some(&dict! {"na" => Na}));
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
    let dict: Dict = dict!["a" => "a", "b" => 100, "c" => true];

    assert!(dict.filter(&Filter::try_from("a and b == 100").unwrap()));
}

/// Helper: assert values_mut works correctly for any Dict (small-vec or tree).
fn check_values_mut(mut dict: Dict) {
    // Collect original values for comparison
    let original: Vec<Value> = dict.values().cloned().collect();

    // Mutate every value to Marker
    for v in dict.values_mut() {
        *v = Value::Marker;
    }

    assert_eq!(dict.len(), original.len());
    assert!(dict.values().all(|v| *v == Value::Marker));
}

#[test]
fn test_dict_values_mut_small() {
    // Stays in small-vec repr (3 entries << 32 threshold)
    let dict: Dict = dict!["a" => "a", "b" => 100, "c" => true];
    check_values_mut(dict);
}

#[test]
fn test_dict_values_mut_tree() {
    // Force tree repr by setting small_max_entries to 0
    let mut dict = Dict::with_small_max_entries(0);
    dict.insert("a".into(), Value::make_str("a"));
    dict.insert("b".into(), 100.into());
    dict.insert("c".into(), true.into());
    check_values_mut(dict);
}

#[test]
fn test_dict_values_mut_size_hint() {
    let mut dict: Dict = dict!["x" => 1, "y" => 2, "z" => 3];
    let mut iter = dict.values_mut();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    iter.next();
    assert_eq!(iter.size_hint(), (2, Some(2)));
}
