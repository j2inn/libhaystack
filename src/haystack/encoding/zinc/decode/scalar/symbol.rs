// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Symbol scalar decoding

use super::super::scanner::Scanner;
use crate::haystack::val::Symbol;
use std::io::{Error, Read};

// Parse Symbol
pub(crate) fn parse_symbol<R: Read>(scanner: &mut Scanner<R>) -> Result<Symbol, Error> {
    scanner.expect_and_consume(b'^')?;

    let mut symbol = Vec::new();

    if !scanner.is_lower() {
        return scanner.make_generic_err("Invalid Symbol start");
    }

    while !scanner.is_eof && (scanner.is_alpha_num() || scanner.is_any_of("~:-._")) {
        symbol.push(scanner.cur);

        if let Err(err) = scanner.read() {
            if !scanner.is_eof {
                return Err(err);
            }
        }
    }

    if symbol.is_empty() {
        return scanner.make_generic_err("Unexpected empty Symbol ");
    }

    Ok(Symbol {
        value: String::from_utf8_lossy(&symbol).to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::parse_symbol;
    use crate::haystack::val::Symbol;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_symbol() {
        let mut input = Cursor::new("^test-symbol.blah".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let symbol = parse_symbol(&mut scanner);
        assert_eq!(symbol.ok(), Some(Symbol::make("test-symbol.blah")));
    }

    #[test]
    fn test_zinc_parse_symbol_bad() {
        let mut input = Cursor::new("^|".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let symbol = parse_symbol(&mut scanner);
        assert!(symbol.is_err());
    }
}
