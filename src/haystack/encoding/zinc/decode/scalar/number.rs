// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Number scalar decoding

use super::super::scanner::Scanner;
use crate::{
    haystack::val::Number,
    units::{get_unit, Unit},
};
use std::io::{Error, Read};

// Parse a Number
pub(crate) fn parse_number<R: Read>(scanner: &mut Scanner<R>) -> Result<Number, Error> {
    let decimal = parse_decimal(scanner)?;

    let mut exponent: Option<String> = None;
    let mut unit: Option<&'static Unit> = None;

    if !scanner.is_eof && scanner.is_any_of("eE") {
        let next = scanner.peek()?;
        if next == b'+' || next == b'-' || (b'0'..=b'9').contains(&next) {
            exponent = Some(parse_exponent(scanner)?);
        }
    }

    if !scanner.is_eof && is_unit_char(scanner) {
        let unit_str = parse_unit(scanner)?;
        unit = get_unit(unit_str.as_str());
        if unit.is_none() {
            return scanner.make_generic_err(&format!("Unit not found: '{unit_str}'"));
        }
    }

    let number = format!(
        "{decimal}{exp}",
        exp = if let Some(e) = exponent { e } else { "".into() }
    );

    match number.parse::<f64>() {
        Ok(num) => Ok(Number { value: num, unit }),
        Err(_) => scanner.make_generic_err(&format!("Invalid number format '{number}'")),
    }
}

// Parse Exponent part of a number
fn parse_exponent<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    scanner.expect_and_consume_any_of("eE")?;

    let sign = if scanner.is_any_of("+-") {
        let sign = String::from_utf8_lossy(&[scanner.cur]).to_string();
        scanner.read()?;
        sign
    } else {
        "".to_string()
    };

    let exponent = parse_decimal(scanner)?;

    Ok(format!("e{sign}{exponent}"))
}

// Parse unit part of a number
fn parse_unit<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    let mut unit = Vec::new();

    while !scanner.is_eof && is_unit_char(scanner) {
        unit.push(scanner.cur);

        if let Err(err) = scanner.read() {
            if !scanner.is_eof {
                return Err(err);
            }
        }
    }

    Ok(String::from_utf8_lossy(&unit).to_string())
}

// Parse Decimal part of a number
pub(crate) fn parse_decimal<R: Read>(scanner: &mut Scanner<R>) -> Result<f64, Error> {
    let mut id = Vec::new();

    while !scanner.is_eof && (scanner.is_digit() || scanner.is_any_of("_.-")) {
        if scanner.cur != b'_' {
            id.push(scanner.cur);
        }

        if let Err(err) = scanner.read() {
            if !scanner.is_eof {
                return Err(err);
            }
        }
    }

    let str = String::from_utf8_lossy(&id).to_string();

    match str.parse::<f64>() {
        Ok(num) => Ok(num),
        Err(err) => {
            scanner.make_generic_err(&format!("Invalid decimal '{str}'. Parse error: {err}"))
        }
    }
}

// Parse negative infinity
pub(crate) fn parse_neg_inf<R: Read>(scanner: &mut Scanner<R>) -> Result<Number, Error> {
    scanner.expect_and_consume(b'-')?;

    scanner.expect_and_consume_seq("INF")?;

    Ok(f64::NEG_INFINITY.into())
}

fn is_unit_char<R: Read>(scanner: &mut Scanner<R>) -> bool {
    scanner.is_alpha() || scanner.is_any_of("$/%_") || scanner.cur > 128
}

#[cfg(test)]
mod test {
    use super::{parse_decimal, parse_neg_inf, parse_number};
    use crate::{haystack::val::Number, units::get_unit_or_default};
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_decimal() {
        let mut input = Cursor::new("120_330.4".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_decimal(&mut scanner);
        assert_eq!(num.ok(), Some(120_330.4))
    }

    #[test]
    fn test_zinc_parse_decimal_bad() {
        let mut input = Cursor::new(".-120_330.4".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_decimal(&mut scanner);
        assert!(num.is_err())
    }

    #[test]
    fn test_zinc_parse_number_neg_inf() {
        let mut input = Cursor::new("-INF".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_neg_inf(&mut scanner);
        assert_eq!(num.ok(), Some(Number::from(f64::NEG_INFINITY)));
        {
            let mut input = Cursor::new("-I".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let num = parse_neg_inf(&mut scanner);
            assert!(num.is_err())
        }
    }

    #[test]
    fn test_zinc_parse_number() {
        let mut input = Cursor::new("42.0e+1%".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_number(&mut scanner);
        assert_eq!(
            num.ok(),
            Some(Number::make_with_unit(42.0e+1, get_unit_or_default("%")))
        );

        {
            let mut input = Cursor::new("1".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let num = parse_number(&mut scanner);
            assert_eq!(num.ok(), Some(Number::make(1.0)));
        }

        {
            let mut input = Cursor::new("-100_233.23".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let num = parse_number(&mut scanner);
            assert_eq!(num.ok(), Some(Number::make(-100233.23)));
        }
    }

    #[test]
    fn test_zinc_parse_number_utf8_unit() {
        let mut input = Cursor::new("1.4m³_gas".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_number(&mut scanner);
        assert_eq!(
            num.ok(),
            Some(Number::make_with_unit(1.4, get_unit_or_default("m³_gas")))
        );
    }

    #[test]
    fn test_zinc_parse_number_bad() {
        let mut input = Cursor::new("42.0e".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let num = parse_number(&mut scanner);
        assert!(num.is_err());
    }
}
