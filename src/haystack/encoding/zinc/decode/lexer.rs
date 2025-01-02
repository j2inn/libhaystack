// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc lexer
//!

use std::io::{Error, Read};

use super::id::{parse_id, parse_literal, Id};
use super::scalar::coord::parse_coord_body;
use super::scalar::date_time::{is_partial_date, parse_date, parse_datetime, parse_time};
use super::scalar::number::{parse_neg_inf, parse_number};
use super::scalar::reference::parse_ref;
use super::scalar::str::parse_str;
use super::scalar::symbol::parse_symbol;
use super::scalar::uri::parse_uri;
use super::scalar::xstr::parse_xstr_body;
use super::scanner::Scanner;
use crate::haystack::val::Value;

///
/// Lexer token value variants
///
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub enum TokenValue {
    Id(Id),
    Value(Value),
    ZincChar(u8),
}

/// The output of the lexer
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct LexerToken {
    pub value: Option<TokenValue>,
}

impl LexerToken {
    pub(crate) fn make_empty() -> Self {
        LexerToken { value: None }
    }

    pub(crate) fn make_id(value: Id) -> Self {
        LexerToken {
            value: Some(TokenValue::Id(value)),
        }
    }

    pub(crate) fn make_value(value: Value) -> Self {
        LexerToken {
            value: Some(TokenValue::Value(value)),
        }
    }

    pub(crate) fn make_char(char: u8) -> Self {
        LexerToken {
            value: Some(TokenValue::ZincChar(char)),
        }
    }
}

///
/// Lexer implementation for Zinc encoding
///
pub struct Lexer<Scanner> {
    scanner: Scanner,
    pub cur: LexerToken,
}

impl<'a, R: Read> Lexer<Scanner<'a, R>> {
    pub fn make(input: &'a mut R) -> Result<Self, Error> {
        Ok(Lexer {
            scanner: Scanner::make(input)?,
            cur: LexerToken::make_empty(),
        })
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.scanner.is_eof
    }

    pub fn read(&mut self) -> Result<&LexerToken, Error> {
        while !self.scanner.is_eof {
            match self.scanner.cur {
                // Spaces
                b' ' | b'\t' => {
                    self.scanner.consume_spaces()?;
                    continue;
                }
                // Str
                b'"' => {
                    let str = parse_str(&mut self.scanner)?;
                    self.cur = LexerToken::make_value(Value::Str(str));
                }
                // Uri
                b'`' => {
                    let uri = parse_uri(&mut self.scanner)?;
                    self.cur = LexerToken::make_value(Value::Uri(uri));
                }
                // Ref
                b'@' => {
                    let ref_ = parse_ref(&mut self.scanner)?;
                    self.cur = LexerToken::make_value(Value::Ref(ref_));
                }
                // Symbol
                b'^' => {
                    let symbol = parse_symbol(&mut self.scanner)?;
                    self.cur = LexerToken::make_value(Value::Symbol(symbol));
                }
                // Special chars
                b',' | b'\r' | b'\n' | b'{' | b'}' | b':' | b'[' | b']' | b'<' | b'>' => {
                    let last_char = self.scanner.cur;
                    // Normalize new lines
                    if last_char == b'\n' || last_char == b'\r' {
                        self.cur = LexerToken::make_char(b'\n');
                    } else {
                        self.cur = LexerToken::make_char(last_char);
                    }

                    return match self.scanner.read() {
                        Ok(char) => {
                            // If using CRLF, normalize to LF
                            if last_char == b'\r' && char == b'\n' {
                                self.scanner.read()?;
                            }

                            Ok(&self.cur)
                        }
                        Err(err) => {
                            if self.scanner.is_eof {
                                Ok(&self.cur)
                            } else {
                                Err(err)
                            }
                        }
                    };
                }
                // Number, Time, Date, DateTime
                b'0'..=b'9' | b'-' => {
                    self.cur = parse_number_date_time(&mut self.scanner)?;
                }
                // Keywords
                b'A'..=b'Z' => {
                    let literal = parse_literal(&mut self.scanner)?;

                    if self.scanner.cur == b'(' {
                        if literal == "C" {
                            let coord = parse_coord_body(&mut self.scanner)?;
                            self.cur = LexerToken::make_value(Value::Coord(coord));
                        } else {
                            let xstr = parse_xstr_body(&literal, &mut self.scanner)?;
                            self.cur = LexerToken::make_value(Value::XStr(xstr));
                        }
                        return Ok(&self.cur);
                    }

                    // Literals
                    self.cur = match literal.as_str() {
                        "M" => Ok(LexerToken::make_value(Value::Marker)),
                        "R" => Ok(LexerToken::make_value(Value::Remove)),
                        "T" => Ok(LexerToken::make_value(Value::make_true())),
                        "F" => Ok(LexerToken::make_value(Value::make_false())),
                        "N" => Ok(LexerToken::make_value(Value::Null)),
                        "NA" => Ok(LexerToken::make_value(Value::Na)),
                        "NaN" => Ok(LexerToken::make_value(Value::make_number(f64::NAN))),
                        "INF" => Ok(LexerToken::make_value(Value::make_number(f64::INFINITY))),
                        _ => self
                            .scanner
                            .make_generic_err(&format!("Unexpected lexer literal '{literal}'")),
                    }?;
                }
                // Ids
                b'a'..=b'z' => {
                    let id = parse_id(&mut self.scanner)?;
                    self.cur = LexerToken::make_id(id);
                }
                _ => {
                    return self.scanner.make_generic_err(&format!(
                        "Unexpected lexer character '{}'",
                        self.scanner.cur as char
                    ))
                }
            };
            return Ok(&self.cur);
        }
        self.cur = LexerToken::make_empty();
        Ok(&self.cur)
    }

    pub(crate) fn make_generic_err<T>(&self, msg: &str) -> Result<T, Error> {
        self.scanner.make_generic_err(msg)
    }

    pub(crate) fn expect_value(&self) -> Result<Value, Error> {
        match &self.cur.value {
            Some(TokenValue::Value(value)) => Ok(value.clone()),
            _ => self.scanner.make_generic_err(&format!(
                "Expected Value token, found token {cur:?}",
                cur = &self.cur.value
            )),
        }
    }

    pub(crate) fn expect_id(&self) -> Result<Id, Error> {
        match &self.cur.value {
            Some(TokenValue::Id(value)) => Ok(value.clone()),
            _ => self.scanner.make_generic_err(&format!(
                "Expected Id token, found token {cur:?}",
                cur = &self.cur.value
            )),
        }
    }

    pub(crate) fn is_id(&self) -> bool {
        matches!(&self.cur.value, Some(TokenValue::Id(_)))
    }

    pub(crate) fn is_char(&self, char: u8) -> bool {
        match &self.cur.value {
            Some(TokenValue::ZincChar(val)) => val == &char,
            _ => false,
        }
    }

    pub(crate) fn expect_char(&self, char: u8, msg: &str) -> Result<u8, Error> {
        match &self.cur.value {
            Some(TokenValue::ZincChar(val)) => {
                if val == &char {
                    Ok(char)
                } else {
                    self.scanner.make_generic_err(&format!(
                        "{msg}. Expected char token {expect:?}, found token {cur:?}",
                        expect = char as char,
                        cur = *val as char
                    ))
                }
            }
            _ => self.scanner.make_generic_err(&format!(
                "{msg}. Expected char token {expect:?}, found token {cur:?}",
                expect = char as char,
                cur = &self.cur.value
            )),
        }
    }

    pub(crate) fn consume_white_spaces(&mut self) -> Result<(), Error> {
        self.scanner.consume_white_spaces()
    }
}

impl<R: Read> Iterator for Lexer<Scanner<'_, R>> {
    type Item = Result<LexerToken, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.scanner.is_eof {
            match self.read() {
                Ok(token) => token.value.as_ref().map(|_| Ok(token.clone())),
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
}

/// Generic parsing over what might be a Number or Date, Time or DateTime
pub(crate) fn parse_number_date_time<R: Read>(
    scanner: &mut Scanner<R>,
) -> Result<LexerToken, Error> {
    if scanner.cur == b'-' {
        if scanner.peek()? == b'I' {
            let inf = parse_neg_inf(scanner)?;
            Ok(LexerToken::make_value(Value::Number(inf)))
        } else {
            let number = parse_number(scanner)?;
            Ok(LexerToken::make_value(Value::Number(number)))
        }
    } else {
        let mut read_count = 0;
        let mut cur = scanner.cur;
        for _ in 0..4 {
            if !cur.is_ascii_digit() || scanner.is_eof {
                break;
            }
            read_count += 1;

            match scanner.peek() {
                Ok(val) => cur = val,
                Err(err) => {
                    if !scanner.is_eof {
                        return Err(err);
                    }
                }
            }
        }

        if scanner.is_eof {
            scanner.is_eof = false;
            let number = parse_number(scanner)?;
            Ok(LexerToken::make_value(Value::Number(number)))
        } else if read_count == 2 && scanner.last_peek == b':' {
            let time = parse_time(scanner)?;
            Ok(LexerToken::make_value(Value::Time(time)))
        } else if read_count == 4 && scanner.last_peek == b'-' && is_partial_date(scanner)? {
            match scanner.peek() {
                Ok(peek) => {
                    if peek != b'T' {
                        let date = parse_date(scanner)?;
                        Ok(LexerToken::make_value(Value::Date(date)))
                    } else {
                        let datetime = parse_datetime(scanner)?;
                        Ok(LexerToken::make_value(Value::DateTime(datetime)))
                    }
                }
                Err(err) => {
                    if scanner.is_eof {
                        let date = parse_date(scanner)?;
                        Ok(LexerToken::make_value(Value::Date(date)))
                    } else {
                        Err(err)
                    }
                }
            }
        } else {
            let number = parse_number(scanner)?;
            Ok(LexerToken::make_value(Value::Number(number)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, LexerToken};
    use crate::haystack::val::{Date, Time, Value};
    use std::io::{Cursor, Error};
    use std::str::FromStr;

    #[test]
    fn test_zinc_lexer_make() {
        let mut input = Cursor::new(" ".as_bytes());

        assert!(Lexer::make(&mut input).is_ok(), "Should create lexer");
    }

    #[test]
    fn test_zinc_lexer_read_value() {
        {
            let mut input = Cursor::new("NA".as_bytes());
            let mut lexer = Lexer::make(&mut input).expect("Lexer");

            let id = lexer.read();
            assert_eq!(id.ok(), Some(&LexerToken::make_value(Value::Na)))
        };
    }

    #[test]
    fn test_zinc_lexer_iterator() {
        {
            let mut input = Cursor::new("T F  N ".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![
                    LexerToken::make_value(Value::make_true()),
                    LexerToken::make_value(Value::make_false()),
                    LexerToken::make_value(Value::Null)
                ])
            )
        };

        {
            let mut input = Cursor::new("NA R INF".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![
                    LexerToken::make_value(Value::Na),
                    LexerToken::make_value(Value::Remove),
                    LexerToken::make_value(Value::make_number(f64::INFINITY))
                ])
            )
        };

        {
            let mut input = Cursor::new("42".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![LexerToken::make_value(Value::make_number(42.0))])
            )
        };

        {
            let mut input = Cursor::new("1234".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![LexerToken::make_value(Value::make_number(1234.0))])
            )
        };

        {
            let mut input = Cursor::new("2021-07-06".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![LexerToken::make_value(Value::make_date(
                    Date::from_str("2021-07-06").expect("Date")
                ))])
            )
        };

        {
            let mut input = Cursor::new("12:23:23".as_bytes());
            let lexer = Lexer::make(&mut input).expect("Lexer");

            let id: Result<Vec<LexerToken>, Error> = lexer.into_iter().collect();
            assert_eq!(
                id.ok(),
                Some(vec![LexerToken::make_value(Value::make_time(
                    Time::from_str("12:23:23").expect("Time")
                ))])
            )
        };
    }
}
