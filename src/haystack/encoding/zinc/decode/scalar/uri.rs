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
                b'u' => {
                    scanner.read()?; // u
                    let unicode = parse_str_unicode_escape(scanner)?;
                    str.extend_from_slice(unicode.as_bytes());
                }
                _ => {
                    return scanner.make_generic_err(&format!(
                        "Invalid Uri escape sequence '{}'",
                        next as char
                    ));
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

    // Reuse the buffer directly when valid UTF-8 (the common case) instead of
    // cloning via `from_utf8_lossy(..).into_owned()`.
    let value = String::from_utf8(str)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());

    Ok(Uri::from(value))
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

    fn parse(input: &str) -> Result<Uri, std::io::Error> {
        let mut input = Cursor::new(input.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");
        parse_uri(&mut scanner)
    }

    #[test]
    fn test_zinc_parse_uri_unicode_escape() {
        assert_eq!(parse(r#"`/f\u1ff6o`"#).ok(), Some(Uri::make("/fῶo")));
        assert_eq!(parse(r#"`a\u{1F600}b`"#).ok(), Some(Uri::make("a😀b")));
        assert_eq!(parse(r#"`a\ud83d\ude00b`"#).ok(), Some(Uri::make("a😀b")));
        assert_eq!(parse(r#"`a😀b`"#).ok(), Some(Uri::make("a😀b")));
        assert!(parse(r#"`a\x`"#).is_err());
        assert!(parse(r#"`a\nb`"#).is_err());
        assert!(parse(r#"`a\ud83d`"#).is_ok());
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
