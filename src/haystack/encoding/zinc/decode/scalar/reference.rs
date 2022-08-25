// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Ref scalar decoding

use super::super::scalar::str::parse_str;
use super::super::scanner::Scanner;
use crate::haystack::val::Ref;
use std::io::{Error, Read};

// Parse Ref
pub(crate) fn parse_ref<R: Read>(scanner: &mut Scanner<R>) -> Result<Ref, Error> {
    scanner.expect_and_consume(b'@')?;

    let mut ref_chars = Vec::new();

    while !scanner.is_eof && (scanner.is_alpha_num() || scanner.is_any_of("~:-._")) {
        ref_chars.push(scanner.cur);

        scanner.advance()?
    }

    if ref_chars.is_empty() {
        return scanner.make_generic_err("Unexpected empty Ref");
    }

    let mut dis: Option<String> = None;
    if !scanner.is_eof && scanner.cur == b' ' && scanner.peek()? == b'"' {
        scanner.read()?;
        dis = Some(parse_str(scanner)?.value);
    }

    Ok(Ref {
        value: String::from_utf8_lossy(&ref_chars).to_string(),
        dis,
    })
}

#[cfg(test)]
mod test {
    use super::parse_ref;
    use crate::haystack::val::Ref;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_ref() {
        let mut input = Cursor::new("@foo-bar_baz.zoom \"a dis\"".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let ref_ = parse_ref(&mut scanner);
        assert_eq!(
            ref_.ok(),
            Some(Ref::make("foo-bar_baz.zoom", Some("a dis")))
        );
        {
            let mut input = Cursor::new("@foo".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let str = parse_ref(&mut scanner);
            assert_eq!(str.ok(), Some(Ref::make("foo", None)));
        }
    }

    #[test]
    fn test_zinc_parse_ref_bad() {
        let mut input = Cursor::new("@".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let ref_ = parse_ref(&mut scanner);
        assert!(ref_.is_err());
        {
            let mut input = Cursor::new("@a \"".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let ref_ = parse_ref(&mut scanner);
            assert!(ref_.is_err());
        }
    }
}
