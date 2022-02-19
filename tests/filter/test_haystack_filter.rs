// Copyright (C) 2020 - 2022, J2 Innovations

//! Tests the Haystack Filter

#[cfg(test)]
use libhaystack::filter::{Filter, Filtered, ListFiltered};
use libhaystack::{dict, val::*};

#[test]
fn test_filter_parse_simple_path() {
    let filter = Filter::try_from("id").expect("Filter");
    assert!(dict! {"id"=> Value::make_marker()}.filter(&filter));
}

#[test]
fn test_filter_parse_simple_or() {
    let filter = &Filter::try_from("id or missing").unwrap();
    println!("{:?}", filter);

    assert!(dict! {"id"=> Value::make_marker()}.filter(&filter));
    assert!(dict! {"missing"=> Value::make_marker()}.filter(&filter));
}

#[test]
fn test_filter_parse_simple_and() {
    let filter = &Filter::try_from("foo and bar").unwrap();
    assert!(dict! {"foo"=> Value::make_marker(), "bar" => Value::make_int(1)}.filter(&filter));
}

#[test]
fn test_filter_parse_simple_and_not() {
    let filter = &Filter::try_from("foo and bar and not baz").unwrap();
    assert!(dict! {"foo"=> Value::make_marker(), "bar" => Value::make_int(1)}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_num() {
    let filter = &Filter::try_from("foo and num == 1").unwrap();
    assert!(dict! {"foo"=> Value::make_marker(), "num" => Value::make_int(1)}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_bool() {
    let filter = &Filter::try_from("bool == true").unwrap();
    assert!(dict! {"bool"=> Value::make_bool(true)}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_ref() {
    let filter = &Filter::try_from("ref ==@ref").unwrap();
    assert!(dict! {"ref" => Value::make_ref("ref")}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_string() {
    let filter = &Filter::try_from(r#"str == "string value""#).unwrap();
    assert!(dict! {"str" => Value::make_str("string value")}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_uri() {
    let filter = &Filter::try_from("uri == `/uri/`").unwrap();
    assert!(dict! {"uri" => Value::make_uri("/uri/")}.filter(&filter));
}

#[test]
fn test_filter_parse_cpm_eq_date() {
    let filter = &Filter::try_from("date == 2020-05-18").unwrap();
    assert!(
        dict! {"date" => Value::make_date(Date::from_ymd(2020, 05, 18).expect("Invalid date"))}
            .filter(&filter)
    );
}

#[test]
fn test_filter_parse_cpm_eq_time() {
    let filter = &Filter::try_from("time == 12:55:22").unwrap();
    assert!(
        dict! {"time" => Value::make_time(Time::from_hms(12, 55, 22).expect("Time"))}
            .filter(&filter)
    );
}

#[test]
fn test_filter_parse_cpm_noteq() {
    let filter = &Filter::try_from("foo and bool != false").unwrap();
    assert!(dict! {"foo"=> Value::make_marker(), "bool" => Value::make_bool(true)}.filter(&filter));

    let filter = &Filter::try_from("num != 42").unwrap();
    assert!(dict! {"num"=> Value::make_int(24)}.filter(&filter));
}

#[test]
fn test_filter_parse_gt_num() {
    let filter = &Filter::try_from("num > 12.5").unwrap();
    assert!(dict! {"num" => Value::make_number(13.0)}.filter(&filter));
}

#[test]
fn test_filter_parse_gte_num() {
    let filter = &Filter::try_from("num >= 42.0").unwrap();
    assert!(dict! {"num" => Value::make_number(42.0)}.filter(&filter));
}

#[test]
fn test_filter_parse_lt_num() {
    let filter = &Filter::try_from("num < 42").unwrap();
    assert!(dict! {"num" => Value::make_number(41.0)}.filter(&filter));
}

#[test]
fn test_filter_parse_lte_num() {
    let filter = &Filter::try_from("num <= 42").unwrap();
    assert!(dict! {"num" => Value::make_number(42.0)}.filter(&filter));
}

#[test]
fn test_filter_parse_gt_bool() {
    let filter = &Filter::try_from("bool > false").unwrap();
    assert!(dict! {"bool" => Value::make_bool(true)}.filter(&filter));
}

#[test]
fn test_filter_parse_gt_date() {
    let filter = &Filter::try_from("date > 2020-05-18").unwrap();
    assert!(
        dict! {"date" => Value::make_date(Date::from_ymd(2020, 06, 18).expect("Invalid Date"))}
            .filter(&filter)
    );
}

#[test]
fn test_filter_parse_path() {
    let filter = &Filter::try_from("dict->val == 1").unwrap();
    assert!(
        dict! {"dict" => Value::make_dict(dict! {"val" => Value::make_int(1)})}.filter(&filter)
    );
}

#[test]
fn test_filter_parse_path_nested() {
    let filter = &Filter::try_from("dict->ref->val == 100").unwrap();
    assert!(dict! {"dict" => Value::make_dict(
        dict! {"ref" => Value::make_dict(dict!{"val" => Value::make_int(100)})}
    )}
    .filter(&filter));
}

#[test]
fn test_filter_parse_parens() {
    let filter = &Filter::try_from("((a or b)) and (c or d)").unwrap();
    assert!(dict! {"a" => Value::make_marker(), "d" => Value::make_marker()}.filter(&filter));
}

#[test]
fn test_filter_or_and() {
    let rows = vec![
        dict! {"marker" => Value::Marker, "value" => Value::Bool(true.into()) },
        dict! {"point" => Value::Marker},
    ];

    let grid = Grid::make_from_dicts(rows);
    let filter = &Filter::try_from("point or (marker and value == true)").unwrap();
    assert_eq!(grid.filter_all(filter).len(), 2);
}

#[test]
fn test_filter_parse_err() {
    let filter = &Filter::try_from("dict->");
    assert!(filter.is_err());

    let filter = &Filter::try_from("o or");
    assert!(filter.is_err());

    let filter = &Filter::try_from("a and");
    assert!(filter.is_err());
}
