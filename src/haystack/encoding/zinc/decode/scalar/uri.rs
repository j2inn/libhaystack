// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Uri scalar decoding

use super::super::scalar::str::parse_str_unicode_escape;
use super::super::scanner::Scanner;
use crate::haystack::val::Uri;
use std::io::{Error, Read};

// Parse Uri
pub(crate) fn parse_uri<R: Read>(scanner: &mut Scanner<R>) -> Result<Uri, Error> {
    let start = scanner.pos;
    scanner.expect_and_consume(b'`')?;

    let mut str = Vec::new();

    while scanner.cur != b'`' {
        if scanner.is_eof {
            return scanner.make_generic_err("Expected '`'");
        }

        if scanner.cur == b'\\' {
            let next = scanner.peek()?;
            match next {
                b':' | b'/' | b'?' | b'#' | b'\\' => {
                    str.push(scanner.cur);
                    str.push(next);
                    scanner.read()?;
                }
                b'[' | b']' | b'@' | b'`' | b'&' | b'=' | b';' => {
                    str.push(next);
                    scanner.read()?;
                }
                _ => {
                    let unicode = parse_str_unicode_escape(scanner)?;
                    str.extend_from_slice(unicode.as_bytes());
                }
            };
        } else {
            str.push(scanner.cur);
        }
        scanner.advance()?
    }

    if start == scanner.pos {
        return scanner.make_generic_err("Unterminated Uri");
    }

    scanner.advance()?;

    Ok(Uri {
        value: String::from_utf8_lossy(&str).to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::parse_uri;
    use crate::haystack::val::Uri;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_uri() {
        let mut input = Cursor::new("`/foo/bar/baz`".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_uri(&mut scanner);
        assert_eq!(str.ok(), Some(Uri::make("/foo/bar/baz")));
        {
            let mut input = Cursor::new("`/\\@\\#`".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let str = parse_uri(&mut scanner);
            assert_eq!(str.ok(), Some(Uri::make("/@\\#")));
        }
    }

    #[test]
    fn test_zinc_parse_uri_bad() {
        let mut input = Cursor::new("`".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert!(parse_uri(&mut scanner).is_err());

        let mut input = Cursor::new("`/foo/bar/baz".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_uri(&mut scanner);
        assert!(str.is_err());
        {
            let mut input = Cursor::new("/foo/bar/baz".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let str = parse_uri(&mut scanner);
            assert!(str.is_err());
        }
    }
}
