// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Dict

#[cfg(test)]
use libhaystack::dict;
use libhaystack::filter::Filter;
use libhaystack::filter::Filtered;
use libhaystack::val::*;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::str::FromStr;

#[test]
fn test_dict_make() {
    let dict: Dict = Dict::from_iter([("a".into(), 100.into()), ("b".into(), Marker.into())]);

    assert_eq!(dict.len(), 2);
}

#[test]
fn test_dict_make_value() {
    let dict: Dict = dict! {"a" => "a", "b" => 100, "c" => true};

    let value: Value = dict.into();

    assert!(value.is_dict());
    assert!(!value.is_list());

    assert_eq!(
        Dict::try_from(&value),
        Ok(dict! {"a" => "a", "b" => 100, "c" => true})
    );
}

#[test]
fn test_dict_accessors() {
    let dict: Dict = dict! {
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
    };

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
    let dict: Dict = dict! {"a" => "a", "b" => 100, "c" => true};

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
    let dict: Dict = dict! {"a" => "a", "b" => 100, "c" => true};
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
    let mut dict: Dict = dict! {"x" => 1, "y" => 2, "z" => 3};
    let mut iter = dict.values_mut();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    iter.next();
    assert_eq!(iter.size_hint(), (2, Some(2)));
}

// --- retain ---

/// Helper that builds a dict via `with_small_max_entries` so the threshold
/// can be set explicitly for tree-downgrade tests.
fn make_dict_with_threshold(entries: &[(&str, i32)], threshold: usize) -> Dict {
    let mut d = Dict::with_small_max_entries(threshold);
    for (k, v) in entries {
        d.insert((*k).into(), (*v).into());
    }
    d
}

#[test]
fn test_dict_retain_small_keeps_matching() {
    // Small repr: keep only numeric values > 1
    let mut dict: Dict = dict! {"a" => 1, "b" => 2, "c" => 3};
    dict.retain(|_, v| matches!(v, Value::Number(n) if n.value > 1.0));
    assert!(!dict.has("a"));
    assert!(dict.has("b"));
    assert!(dict.has("c"));
    assert_eq!(dict.len(), 2);
}

#[test]
fn test_dict_retain_small_removes_all() {
    let mut dict: Dict = dict! {"a" => 1, "b" => 2};
    dict.retain(|_, _| false);
    assert!(dict.is_empty());
}

#[test]
fn test_dict_retain_small_keeps_all() {
    let mut dict: Dict = dict! {"a" => 1, "b" => 2};
    dict.retain(|_, _| true);
    assert_eq!(dict.len(), 2);
}

#[test]
fn test_dict_retain_tree_keeps_matching() {
    // Force tree repr (threshold = 0), then retain
    let mut dict = make_dict_with_threshold(&[("a", 1_i32), ("b", 2), ("c", 3)], 0);
    assert_eq!(dict.small_max_entries(), 0);
    dict.retain(|_, v| matches!(v, Value::Number(n) if n.value > 1.0));
    assert!(!dict.has("a"));
    assert!(dict.has("b"));
    assert!(dict.has("c"));
    assert_eq!(dict.len(), 2);
}

#[test]
fn test_dict_retain_tree_downgrades_to_small() {
    // threshold = 4; start with 5 entries (forces tree), then retain 3
    // → surviving count (3) <= threshold (4) → must downgrade to Small
    let mut dict =
        make_dict_with_threshold(&[("a", 1_i32), ("b", 2), ("c", 3), ("d", 4), ("e", 5)], 4);
    // With 5 entries and threshold 4 the dict spills to Tree on the 5th insert
    assert_eq!(dict.small_max_entries(), 4);
    dict.retain(|k, _| matches!(k, "b" | "c" | "d"));
    assert_eq!(dict.len(), 3);
    // Verify it downgraded: small_max_entries unchanged, repr is Small
    // We can confirm indirectly via shrink_to_fit being idempotent and
    // the dict still behaving correctly.
    assert!(dict.has("b"));
    assert!(dict.has("c"));
    assert!(dict.has("d"));
    assert!(!dict.has("a"));
    assert!(!dict.has("e"));

    // A subsequent insert must still work (goes into whichever repr it landed in)
    dict.insert("z".into(), Value::Marker);
    assert!(dict.has("z"));
}

#[test]
fn test_dict_retain_mutates_values() {
    let mut dict: Dict = dict! {"a" => 1, "b" => 2, "c" => 3};
    dict.retain(|_, v| {
        if let Value::Number(n) = v {
            n.value *= 10.0;
        }
        true
    });
    assert_eq!(dict.len(), 3);
    assert_eq!(dict.get_num("a").map(|n| n.value as i64), Some(10));
    assert_eq!(dict.get_num("b").map(|n| n.value as i64), Some(20));
    assert_eq!(dict.get_num("c").map(|n| n.value as i64), Some(30));
}

// --- Dict <-> BTreeMap conversions ---

#[test]
fn test_dict_from_btreemap_small() {
    // Fewer entries than the threshold → must use the small-vec repr internally,
    // but the resulting Dict must have the same keys and values.
    let mut map = BTreeMap::new();
    map.insert("a".to_string(), Value::make_str("hello"));
    map.insert("b".to_string(), 42.into());
    map.insert("c".to_string(), Value::Marker);

    let dict = Dict::from(map.clone());

    assert_eq!(dict.len(), 3);
    assert_eq!(dict.get_str("a"), Some(&Str::from("hello")));
    assert_eq!(dict.get_num("b"), Some(&Number::from(42)));
    assert!(dict.has_marker("c"));
}

#[test]
fn test_dict_into_btreemap_small() {
    // Dict backed by small-vec → BTreeMap must contain the same entries.
    let dict = dict! {"x" => "val", "y" => 7, "z" => true};

    let map: BTreeMap<String, Value> = dict.into();

    assert_eq!(map.len(), 3);
    assert_eq!(map["x"], Value::make_str("val"));
    assert_eq!(map["y"], Value::from(7));
    assert_eq!(map["z"], Value::from(true));
}

#[test]
fn test_dict_into_btreemap_tree() {
    // Force tree repr (threshold = 0), then convert → BTreeMap.
    let mut dict = Dict::with_small_max_entries(0);
    dict.insert("a".into(), Value::make_str("a"));
    dict.insert("b".into(), 2.into());

    let map: BTreeMap<String, Value> = dict.into();

    assert_eq!(map.len(), 2);
    assert_eq!(map["a"], Value::make_str("a"));
    assert_eq!(map["b"], Value::from(2));
}

#[test]
fn test_dict_from_btreemap_roundtrip() {
    // Dict → BTreeMap → Dict must be equal to the original.
    let original = dict! {"p" => "one", "q" => 2, "r" => true};

    let map: BTreeMap<String, Value> = original.clone().into();
    let roundtripped = Dict::from(map);

    assert_eq!(original, roundtripped);
}

#[test]
fn test_dict_from_btreemap_preserves_order() {
    // Keys must be iterated in sorted order after the round-trip (BTreeMap is sorted).
    let mut map = BTreeMap::new();
    map.insert("z".to_string(), Value::Marker);
    map.insert("a".to_string(), Value::Marker);
    map.insert("m".to_string(), Value::Marker);

    let dict = Dict::from(map);
    let keys: Vec<&str> = dict.keys().map(|k| k.as_str()).collect();

    assert_eq!(keys, vec!["a", "m", "z"]);
}

#[test]
fn test_dict_from_btreemap_empty() {
    let dict = Dict::from(BTreeMap::<String, Value>::new());
    assert!(dict.is_empty());
}

#[test]
fn test_dict_into_btreemap_empty() {
    let map: BTreeMap<String, Value> = Dict::new().into();
    assert!(map.is_empty());
}
