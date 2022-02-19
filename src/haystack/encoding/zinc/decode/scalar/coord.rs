// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Coord scalar decoding

use super::super::scalar::number::parse_decimal;
use super::super::scanner::Scanner;
use crate::haystack::val::Coord;
use std::io::{Error, Read};

// Parse Coord body
pub(in crate::haystack::encoding::zinc::decode) fn parse_coord_body<R: Read>(
    scanner: &mut Scanner<R>,
) -> Result<Coord, Error> {
    scanner.expect_and_consume(b'(')?;

    scanner.consume_spaces()?;

    let lat = parse_decimal(scanner)?;

    scanner.consume_spaces()?;
    scanner.expect_and_consume(b',')?;
    scanner.consume_spaces()?;

    let long = parse_decimal(scanner)?;
    scanner.consume_spaces()?;
    scanner.expect_and_consume(b')')?;

    Ok(Coord::make(lat, long))
}

#[cfg(test)]
mod test {
    use super::parse_coord_body;
    use crate::haystack::val::Coord;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_coord() {
        let mut input = Cursor::new("(1.0, 2.0)".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let coord = parse_coord_body(&mut scanner);
        assert_eq!(coord.ok(), Some(Coord::make(1.0, 2.0)))
    }

    #[test]
    fn test_zinc_parse_decimal_bad() {
        let mut input = Cursor::new("1.0, 2.0)".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let coord = parse_coord_body(&mut scanner);
        assert!(coord.is_err())
    }
}
