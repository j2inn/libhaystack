// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc XStr scalar decoding

use super::super::scalar::str::parse_str;
use super::super::scanner::Scanner;
use crate::haystack::val::XStr;
use std::io::{Error, Read};

// Parse XStr
pub(crate) fn parse_xstr_body<R: Read>(
    name: &str,
    scanner: &mut Scanner<R>,
) -> Result<XStr, Error> {
    scanner.expect_and_consume(b'(')?;
    scanner.consume_spaces()?;

    let value = parse_str(scanner)?;

    scanner.consume_spaces()?;
    scanner.expect_and_consume(b')')?;

    Ok(XStr::make(name, value.as_str()))
}

#[cfg(test)]
mod test {
    use super::parse_xstr_body;
    use crate::haystack::val::XStr;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_xstr() {
        let mut input = Cursor::new(r#"( "a type" )"#.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let xstr = parse_xstr_body("Foo", &mut scanner);
        assert_eq!(xstr.ok(), Some(XStr::make("Foo", "a type")));
    }

    #[test]
    fn test_zinc_parse_xstr_bad() {
        let mut input = Cursor::new(r#"("bad""#.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_xstr_body("Bad", &mut scanner);
        assert!(str.is_err())
    }
}
