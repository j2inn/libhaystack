// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Str scalar decoding

use super::super::scanner::Scanner;
use crate::haystack::val::Str;
use std::io::{Error, Read};

// Parse a Str
pub(crate) fn parse_str<R: Read>(scanner: &mut Scanner<R>) -> Result<Str, Error> {
    let start = scanner.pos;
    scanner.expect_and_consume(b'"')?;

    let mut str = Vec::new();

    while scanner.cur != b'"' {
        if scanner.is_eof {
            return scanner.make_generic_err("Expected '\"'");
        }

        if scanner.cur == b'\\' {
            let unicode = parse_str_escape(scanner)?;
            str.extend_from_slice(unicode.as_bytes());
        } else {
            str.push(scanner.cur);
        }
        if let Err(err) = scanner.read() {
            if !scanner.is_eof {
                return Err(err);
            }
        }
    }

    if start == scanner.pos {
        return scanner.make_generic_err("Unterminated Str");
    }

    if let Err(err) = scanner.read() {
        if !scanner.is_eof {
            return Err(err);
        }
    }

    Ok(Str {
        value: String::from_utf8_lossy(&str).to_string(),
    })
}

// Parse a Str escape sequence
pub(super) fn parse_str_escape<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    scanner.read()?;

    match scanner.cur {
        b'b' => Ok("\x0b".into()),
        b'f' => Ok("\x0f".into()),
        b'n' => Ok("\n".into()),
        b'r' => Ok("\r".into()),
        b't' => Ok("\t".into()),
        b'"' => Ok("\"".into()),
        b'$' => Ok("$".into()),
        b'\'' => Ok("'".into()),
        b'`' => Ok("`".into()),
        b'\\' => Ok("\\".into()),
        b'u' => Ok(parse_str_unicode_escape(scanner)?),
        _ => scanner.make_generic_err(&format!("Invalid escape sequence '{}'", scanner.cur)),
    }
}

// Parse a Str unicode char escape sequence
pub(super) fn parse_str_unicode_escape<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    if scanner.cur == b'u' {
        let mut esc = Vec::new();

        for _ in 0..4 {
            scanner.read()?;

            if !scanner.is_hex_digit() {
                return scanner
                    .make_generic_err(&format!("Invalid unicode escape '{}'", scanner.cur));
            }
            esc.push(scanner.cur)
        }

        let str = String::from_utf8_lossy(&esc).to_string();
        match u16::from_str_radix(&str, 16) {
            Ok(char) => Ok(String::from_utf16_lossy(&[char])),
            Err(err) => scanner
                .make_generic_err(&format!("Invalid unicode escape char '{str}', err: {err}")),
        }
    } else {
        scanner.make_generic_err(&format!("Invalid escape sequence '{}'", scanner.cur))
    }
}

#[cfg(test)]
mod test {
    use super::parse_str;
    use crate::haystack::val::Str;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_str() {
        let mut input = Cursor::new(r#""A String"  "#.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_str(&mut scanner);
        assert_eq!(str.ok(), Some(Str::make("A String")));

        {
            let mut input = Cursor::new("\"\n\t\"".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let str = parse_str(&mut scanner);
            assert_eq!(str.ok(), Some(Str::make("\n\t")))
        }
    }

    #[test]
    fn test_zinc_parse_utf8_str() {
        let mut input = Cursor::new("\"m³_gas\"".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_str(&mut scanner);
        assert_eq!(str.ok(), Some(Str::make("m³_gas")));
    }

    #[test]
    fn test_zinc_parse_str_bad() {
        let mut input = Cursor::new(r#"""#.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        assert!(parse_str(&mut scanner).is_err());

        let mut input = Cursor::new(r#""ala bala"#.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_str(&mut scanner);
        assert!(str.is_err());
        {
            let mut input = Cursor::new("\"x".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let str = parse_str(&mut scanner);
            assert!(str.is_err())
        }
    }
}
