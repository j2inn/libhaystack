// Copyright (C) 2020 - 2022, J2 Innovations

//!
//! Units parser
//!

extern crate pom;

use libhaystack::units::Unit;
use libhaystack::units::UnitDimensions;
use pom::char_class::{alpha, alphanum, digit, multispace, space};
use pom::parser::Parser;
use pom::parser::*;

use std::cell::RefCell;
use std::str::FromStr;

thread_local! {
    pub (super) static QUANTITY: RefCell<Option<String>> = RefCell::new(None);
}

fn parse_space<'a>() -> Parser<'a, u8, ()> {
    is_a(space).repeat(0..).discard()
}

fn parse_multi_space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

fn parse_comment<'a>() -> Parser<'a, u8, ()> {
    (seq(b"//") * none_of(b"\n").repeat(0..)).discard()
}

fn parse_quantity<'a>() -> Parser<'a, u8, String> {
    (seq(b"--") * (is_a(alphanum) | is_a(space)).repeat(1..) - none_of(b"\n").repeat(0..))
        .map(|s| String::from_utf8_lossy(&s).trim().into())
}

fn parse_float<'a>() -> Parser<'a, u8, f64> {
    let frac = sym(b'.') + is_a(digit).repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + is_a(digit).repeat(1..);
    let number = sym(b'-').opt() + is_a(digit).repeat(0..) + frac.opt() + exp.opt();
    number
        .collect()
        .convert(std::str::from_utf8)
        .convert(f64::from_str)
}

fn parse_integer<'a>() -> Parser<'a, u8, i64> {
    (sym(b'-').opt() + is_a(digit).repeat(1..))
        .collect()
        .convert(std::str::from_utf8)
        .convert(i64::from_str)
}

fn parse_offset<'a>() -> Parser<'a, u8, f64> {
    parse_float()
}

fn parse_scale<'a>() -> Parser<'a, u8, f64> {
    parse_float()
}

fn parse_exp<'a>() -> Parser<'a, u8, i8> {
    parse_integer().map(|v| v as i8)
}

fn parse_base<'a>() -> Parser<'a, u8, &'a str> {
    (seq(b"kg") | seq(b"mol") | seq(b"m") | seq(b"sec") | seq(b"K") | seq(b"A") | seq(b"cd"))
        .convert(std::str::from_utf8)
}

fn parse_ration<'a>() -> Parser<'a, u8, (&'a str, i8)> {
    parse_base() + parse_exp()
}

fn parse_dim<'a>() -> Parser<'a, u8, Vec<(&'a str, i8)>> {
    list(parse_ration(), sym(b'*'))
}

fn parse_id<'a>() -> Parser<'a, u8, &'a str> {
    (is_a(alpha) | sym(b'_') | sym(b'%') | sym(b'/') | sym(b'$') | is_a(|c| c > 128))
        .repeat(0..)
        .collect()
        .convert(std::str::from_utf8)
}

fn parse_names<'a>() -> Parser<'a, u8, Vec<&'a str>> {
    list(parse_id(), sym(b',') - parse_space()).expect("names")
}

pub(super) fn parse_unit<'a>() -> Parser<'a, u8, Unit> {
    let dims = sym(b';') * parse_space() * parse_dim();
    let scale = sym(b';') * parse_space() * parse_scale();
    let offset = sym(b';') * parse_space() * parse_offset();
    (parse_names() - parse_space() + (dims + (scale + (offset).opt()).opt()).opt()).map(
        |(names, dims)| Unit {
            ids: names.into_iter().map(String::from).collect(),
            dimensions: dims
                .as_ref()
                .map(|(dims, ..)| {
                    dims.iter()
                        .fold(UnitDimensions::default(), |mut cur, (dim, scale)| {
                            match *dim {
                                "kg" => cur.kg = *scale,
                                "m" => cur.m = *scale,
                                "sec" => cur.sec = *scale,
                                "K" => cur.k = *scale,
                                "A" => cur.a = *scale,
                                "mol" => cur.mol = *scale,
                                "cd" => cur.cd = *scale,
                                _ => panic!("Invalid dimension {}", dim),
                            };
                            cur
                        })
                })
                .and_then(|u| {
                    if u == UnitDimensions::default() {
                        None
                    } else {
                        Some(u)
                    }
                }),
            quantity: QUANTITY.with(|tlv| tlv.borrow().clone()),
            scale: dims
                .as_ref()
                .map_or(1.0, |(.., scale)| scale.map_or(1.0, |(scale, ..)| scale)),
            offset: dims.as_ref().map_or(0.0, |(.., scale)| {
                scale.map_or(0.0, |(.., offset)| offset.unwrap_or(0.0))
            }),
        },
    )
}

pub(super) fn parse_units<'a>() -> Parser<'a, u8, Vec<Option<Unit>>> {
    let map_quantity = || {
        parse_quantity().map(|quantity| {
            QUANTITY.with(|tlv| {
                let _ = tlv.borrow_mut().insert(quantity);
            })
        })
    };
    parse_multi_space()
        * parse_comment().opt()
        * list(
            map_quantity().map(|_| None) | parse_comment().map(|_| None) | parse_unit().map(Some),
            sym(b'\n').repeat(1..),
        )
        - end()
}

#[cfg(test)]
mod test {
    use std::vec;

    use libhaystack::units::Unit;
    use libhaystack::units::UnitDimensions;

    use super::parse_unit;
    use super::parse_units;
    use super::QUANTITY;

    #[test]
    fn test_unit_parser_unit_dimensions() {
        let unit = parse_unit()
            .parse("meters_per_second_squared, m/s²; m1*sec-2".as_bytes())
            .expect("Unit");

        assert!(unit.ids.contains(&"meters_per_second_squared".to_string()));
        assert!(unit.ids.contains(&"m/s²".to_string()));

        assert_eq!(unit.scale, 1.0);
        assert_eq!(unit.offset, 0.0);
        assert_eq!(unit.dimensions.as_ref().expect("Dims").m, 1);
        assert_eq!(unit.dimensions.as_ref().expect("Dims").sec, -2);
    }

    #[test]
    fn test_unit_parser_dimensionless_unit_scale() {
        let unit = parse_unit().parse(b"percent, %; ; 0.01").expect("Unit");

        assert!(unit.ids.contains(&"percent".to_string()));
        assert!(unit.ids.contains(&"%".to_string()));

        assert_eq!(unit.scale, 0.01);
        assert_eq!(unit.offset, 0.0);
        assert!(unit.dimensions.is_none());

        let unit = parse_unit()
            .parse(b"degrees_phase, degPh; ; 0.017453292519943")
            .expect("Unit");

        assert!(unit.ids.contains(&"degrees_phase".to_string()));
        assert!(unit.ids.contains(&"degPh".to_string()));
        assert_eq!(unit.scale, 0.017453292519943);
    }

    #[test]
    fn test_unit_parser_complete_unit_def() {
        let unit = parse_unit()
            .parse(
                "fictive_unit,fictive/ft²; kg1*sec-2*A3*mol-1*m99*cd100*K9; 1.135433731957E7;-2.1"
                    .as_bytes(),
            )
            .expect("Unit");

        assert!(unit.ids.contains(&"fictive_unit".to_string()));
        assert!(unit.ids.contains(&"fictive/ft²".to_string()));

        assert_eq!(unit.scale, 1.135433731957E7);
        assert_eq!(unit.offset, -2.1);
        assert_eq!(
            unit.dimensions.expect("Dimensions"),
            UnitDimensions {
                kg: 1,
                m: 99,
                sec: -2,
                a: 3,
                mol: -1,
                cd: 100,
                k: 9
            }
        );
    }

    #[test]
    fn test_unit_parser_units() {
        let units = parse_units()
            .parse(
                concat!(
                    "-- electric potential (kg1*m2*sec-3*A-1) \n",
                    "millivolt, mV; kg1*m2*sec-3*A-1; 0.0010\n",
                    "// Comment Line \n",
                    "megavolt, MV; kg1*m2*sec-3*A-1; 1000000.0"
                )
                .as_bytes(),
            )
            .expect("Units");

        assert_eq!(units.len(), 4);

        QUANTITY.with(|q| assert_eq!(*q.borrow(), Some("electric potential".to_string())));

        assert_eq!(
            units[1].as_ref().unwrap(),
            &Unit {
                ids: vec!["millivolt".to_string(), "mV".to_string()],
                quantity: Some("electric potential".to_string()),
                dimensions: Some(UnitDimensions {
                    kg: 1,
                    m: 2,
                    sec: -3,
                    a: -1,
                    ..UnitDimensions::default()
                }),
                offset: 0.0,
                scale: 0.0010,
            }
        );

        assert_eq!(
            units[3].as_ref().unwrap(),
            &Unit {
                ids: vec!["megavolt".to_string(), "MV".to_string()],
                quantity: Some("electric potential".to_string()),
                dimensions: Some(UnitDimensions {
                    kg: 1,
                    m: 2,
                    sec: -3,
                    a: -1,
                    ..UnitDimensions::default()
                }),
                offset: 0.0,
                scale: 1000000.0,
            }
        );
    }
}
