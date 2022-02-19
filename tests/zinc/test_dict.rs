// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Dict

#[cfg(test)]
use libhaystack::dict;
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;
use std::iter::FromIterator;
use std::str::FromStr;

#[test]
fn test_zinc_dict_encode() {
    let rec = dict! {
        "site" => Value::make_marker(),
        "name" => Value::make_str("Foo"),
        "dict" => Value::make_dict(dict! {"foo" => Value::make_bool(true)})
    };

    let zinc = rec.to_zinc_string();
    assert_eq!(zinc.unwrap(), r#"{dict:{foo:T},name:"Foo",site}"#);

    assert_eq!(
        dict![
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
        ]
        .to_zinc_string()
        .unwrap(),
        "{a:\"a\",b:100,c:T,d:R,e:[\"str\"],f:{na:NA},g:@ref,h:^symbol,i:`uri`,j:Xstr(\"val\"),k:NA,l,m:20:00:00,n:2021-06-19,o:2021-06-19T19:48:23Z,p:C(34.0522,118.2437),q:<<\nver:\"3.0\"\nempty\n>>}",
    )
}
#[test]
fn test_zinc_dict_decode() {
    let value: Value = from_str(r#"{dict:{foo:T, x:M} name:"Foo" site}"#).unwrap();
    let dict = dict! {
        "site" => Value::make_marker(),
        "name" => Value::make_str("Foo"),
        "dict" => Value::make_dict(dict! {"foo" => Value::make_bool(true), "x" => Value::Marker})
    };
    assert_eq!(value, Value::make_dict(dict));

    let zinc_dict = concat!(
        "{",
        r#"a:"a" "#,
        "b:100 ",
        "c:T ",
        "d:R ",
        r#"e:["str"] "#,
        "f:{na:NA} ",
        "g:@ref ",
        "h:^symbol ",
        "i:`uri` ",
        "j:Xstr(\"val\") ",
        "k:NA ",
        "l ",
        "m:20:00:00 ",
        "n:2021-06-19 ",
        "o:2021-06-19T19:48:23Z ",
        "p:C(34.0522,118.2437) ",
        "q:<<\nver:\"3.0\"\nempty\n>>",
        "r:42",
        "}"
    );
    let value = from_str(zinc_dict).unwrap();

    let dict = match value {
        Value::Dict(dict) => dict,
        _ => panic!("Must be a Dict"),
    };

    assert_eq!(dict.len(), 18);
    assert!(('a'..='r').into_iter().all(|n| dict.has(&n.to_string())));

    assert!(dict.missing("x"));

    assert_eq!(dict.get_str("a"), Some(&Str::from("a")));
    assert_eq!(dict.get_num("b"), Some(&Number::from(100)));
    assert_eq!(dict.get_bool("c"), Some(&Bool::from(true)));
    assert_eq!(dict.get_list("e"), Some(&vec!["str".into()]));
    assert_eq!(dict.get_dict("f"), Some(&dict! {"na" => Na.into()}));
    assert_eq!(dict.get_ref("g"), Some(&Ref::from("ref")));
    assert_eq!(dict.get_symbol("h"), Some(&Symbol::from("symbol")));
    assert_eq!(dict.get_uri("i"), Some(&Uri::from("uri")));
    assert_eq!(dict.get_xstr("j"), Some(&XStr::make("Xstr", "val")));
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
    assert_eq!(dict.get_num("r"), Some(&Number::make(42.0)));

    assert!(dict.has_remove("d"));
    assert!(dict.has_na("k"));
    assert!(dict.has_marker("l"));

    assert_eq!(dict["a"], Value::make_str("a"));
}

#[test]
fn test_zinc_dict_comma_decode() {
    let value: Value = from_str(r#"{a,b,c}"#).unwrap();
    assert_eq!(
        value,
        Value::make_dict(dict! {"a"=> Value::Marker, "b"=> Value::Marker,"c"=> Value::Marker})
    );
}
