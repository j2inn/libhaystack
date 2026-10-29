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
        scanner.advance()?
    }

    if start == scanner.pos {
        return scanner.make_generic_err("Unterminated Str");
    }

    scanner.advance()?;

    // Reuse the buffer directly when valid UTF-8 (the common case) instead of
    // cloning via `from_utf8_lossy(..).into_owned()`.
    let value = String::from_utf8(str)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());

    Ok(Str::from(value))
}

// Parse a Str escape sequence
pub(super) fn parse_str_escape<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    scanner.read()?;

    match scanner.cur {
        b'b' => Ok("\x08".into()),
        b'f' => Ok("\x0c".into()),
        b'n' => Ok("\n".into()),
        b'r' => Ok("\r".into()),
        b't' => Ok("\t".into()),
        b'"' => Ok("\"".into()),
        b'$' => Ok("$".into()),
        b'\'' => Ok("'".into()),
        b'`' => Ok("`".into()),
        b'\\' => Ok("\\".into()),
        b'u' => Ok(parse_str_unicode_escape(scanner)?),
        _ => scanner.make_generic_err(&format!(
            "Invalid escape sequence '{}'",
            scanner.cur as char
        )),
    }
}

// Parse a Str unicode char escape sequence, either `\uXXXX` or `\u{X}`.
// Surrogate pairs written as two `\uXXXX` escapes are joined into one char.
pub(super) fn parse_str_unicode_escape<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    if scanner.cur != b'u' {
        return scanner.make_generic_err(&format!(
            "Invalid escape sequence '{}'",
            scanner.cur as char
        ));
    }

    let code = parse_unicode_code(scanner)?;

    if (0xD800..=0xDBFF).contains(&code) && scanner.next_is(b"\\u") {
        scanner.read()?; // \
        scanner.read()?; // u
        let low = parse_unicode_code(scanner)?;

        if (0xDC00..=0xDFFF).contains(&low) {
            let code_point = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
            return Ok(to_char(code_point).to_string());
        }

        return Ok(format!("{}{}", to_char(code), to_char(low)));
    }

    Ok(to_char(code).to_string())
}

// Convert a code point to a char, using the replacement char for lone surrogates.
fn to_char(code: u32) -> char {
    char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER)
}

// Read the code of a `\uXXXX` or `\u{X}` escape. The scanner must be on the `u`
// and is left on the last character of the escape.
fn parse_unicode_code<R: Read>(scanner: &mut Scanner<R>) -> Result<u32, Error> {
    let mut esc = String::new();

    if scanner.next_is(b"{") {
        scanner.read()?; // {

        loop {
            scanner.read()?;

            if scanner.cur == b'}' {
                break;
            }
            if !scanner.is_hex_digit() {
                return scanner.make_generic_err(&format!(
                    "Invalid unicode escape '{esc}{}'",
                    scanner.cur as char
                ));
            }
            if esc.len() == 6 {
                return scanner.make_generic_err(&format!(
                    "Invalid unicode escape '{esc}{}...', must have 1 to 6 hex digits",
                    scanner.cur as char
                ));
            }
            esc.push(scanner.cur as char)
        }

        if esc.is_empty() {
            return scanner
                .make_generic_err("Invalid unicode escape '{}', must have 1 to 6 hex digits");
        }
    } else {
        for _ in 0..4 {
            scanner.read()?;

            if !scanner.is_hex_digit() {
                return scanner.make_generic_err(&format!(
                    "Invalid unicode escape '{}'",
                    scanner.cur as char
                ));
            }
            esc.push(scanner.cur as char)
        }
    }

    match u32::from_str_radix(&esc, 16) {
        Ok(code) if code <= 0x10FFFF => Ok(code),
        Ok(_) => scanner.make_generic_err(&format!("Invalid unicode escape '{esc}', out of range")),
        Err(err) => {
            scanner.make_generic_err(&format!("Invalid unicode escape '{esc}', err: {err}"))
        }
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

    #[test]
    fn test_zinc_parse_str_backspace_and_formfeed_escapes() {
        // \b must decode to backspace (U+0008), \f to form feed (U+000C)
        let mut input = Cursor::new("\"\\b\\f\"".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let str = parse_str(&mut scanner);
        assert_eq!(str.ok(), Some(Str::make("\x08\x0c")));
    }

    fn parse(input: &str) -> Result<Str, std::io::Error> {
        let mut input = Cursor::new(input.as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");
        parse_str(&mut scanner)
    }

    #[test]
    fn test_zinc_parse_str_unicode_escapes() {
        assert_eq!(parse(r#""Hell\u00D3""#).ok(), Some(Str::make("HellÓ")));
        assert_eq!(parse(r#""Hell\u{D3}!""#).ok(), Some(Str::make("HellÓ!")));
        assert_eq!(parse(r#""a\u{1F600}b""#).ok(), Some(Str::make("a😀b")));
        assert_eq!(
            parse(r#""\u{0}\u{10FFFF}""#).ok(),
            Some(Str::make("\0\u{10FFFF}"))
        );
    }

    #[test]
    fn test_zinc_parse_str_surrogate_pairs() {
        assert_eq!(parse(r#""a\ud83d\ude00b""#).ok(), Some(Str::make("a😀b")));
        assert_eq!(
            parse(r#""a\u{D83D}\u{DE00}b""#).ok(),
            Some(Str::make("a😀b"))
        );
        assert_eq!(parse(r#""a😀b""#).ok(), Some(Str::make("a😀b")));

        // Lone and mismatched surrogates become replacement characters.
        assert_eq!(parse(r#""\ud83d""#).ok(), Some(Str::make("\u{FFFD}")));
        assert_eq!(parse(r#""\ud83d\n""#).ok(), Some(Str::make("\u{FFFD}\n")));
        assert_eq!(
            parse(r#""\ud83d\u0041""#).ok(),
            Some(Str::make("\u{FFFD}A"))
        );
    }

    #[test]
    fn test_zinc_parse_str_bad_unicode_escapes() {
        for bad in [
            r#""\u{}""#,
            r#""\u{1234567}""#,
            r#""\u{110000}""#,
            r#""\u{12G}""#,
            r#""\u{12"#,
            r#""\u12G4""#,
            r#""\u12"#,
            r#""\ud83d\"#,
            r#""\ud83d"#,
        ] {
            assert!(parse(bad).is_err(), "{bad} should fail");
        }
    }
}
