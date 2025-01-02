// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter lexing

use super::path::Path;
use crate::haystack::encoding::zinc::decode::id::{parse_id, Id};
use crate::haystack::encoding::zinc::decode::lexer::{
    parse_number_date_time, TokenValue as ZincTokenValue,
};
use crate::haystack::encoding::zinc::decode::scalar::reference::parse_ref;
use crate::haystack::encoding::zinc::decode::scalar::str::parse_str;
use crate::haystack::encoding::zinc::decode::scalar::symbol::parse_symbol;
use crate::haystack::encoding::zinc::decode::scalar::uri::parse_uri;
use crate::haystack::encoding::zinc::decode::scanner::Scanner;
use crate::haystack::val::Value;
use crate::val::Symbol;
use std::io::{Error, Read};

///
/// Filter Lexer token value variants
///
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) enum TokenValue {
    Value(Value),
    Path(Path),
    Rel(Symbol),
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LeftParens,
    RightParens,
    WildcardEq,
}

/// The output of the filter lexer
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct LexerToken {
    pub(super) value: Option<TokenValue>,
}

impl LexerToken {
    pub(super) fn make_empty() -> Self {
        LexerToken { value: None }
    }

    pub(super) fn make_value(value: Value) -> Self {
        LexerToken {
            value: Some(TokenValue::Value(value)),
        }
    }

    pub(crate) fn make_path(value: Path) -> Self {
        LexerToken {
            value: Some(TokenValue::Path(value)),
        }
    }

    pub(crate) fn make(value: TokenValue) -> Self {
        LexerToken { value: Some(value) }
    }
}

///
/// Lexer for Haystack Filter
///
pub(super) struct Lexer<Scanner> {
    scanner: Scanner,
    pub cur: LexerToken,
}

impl<'a, R: Read> Lexer<Scanner<'a, R>> {
    pub(crate) fn make(input: &'a mut R) -> Result<Self, Error> {
        Ok(Lexer {
            scanner: Scanner::make(input)?,
            cur: LexerToken::make_empty(),
        })
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.scanner.is_eof
    }

    pub(crate) fn read(&mut self) -> Result<&LexerToken, Error> {
        while !self.scanner.is_eof {
            match self.scanner.cur {
                // White spaces
                b'\n' | b'\r' | b'\t' | b' ' => {
                    self.scanner.consume_white_spaces()?;
                    continue;
                } // Str
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
                // Number, Time, Date, DateTime
                b'0'..=b'9' | b'-' => {
                    self.cur = if let Some(token) = parse_number_date_time(&mut self.scanner)?.value
                    {
                        match token {
                            ZincTokenValue::Value(value) => match &value {
                                Value::Number(num) => {
                                    if num.value.is_infinite() {
                                        return self
                                            .scanner
                                            .make_generic_err("Unexpected INF token");
                                    } else {
                                        LexerToken::make_value(value)
                                    }
                                }
                                _ => LexerToken::make_value(value),
                            },
                            _ => {
                                return self
                                    .scanner
                                    .make_generic_err(&format!("Unexpected token type: {token:?}"))
                            }
                        }
                    } else {
                        return self.scanner.make_generic_err("Unexpected empty token");
                    }
                }
                // Grouping
                b'(' => {
                    self.scanner.read().ok();
                    self.cur = LexerToken::make(TokenValue::LeftParens)
                }
                b')' => {
                    self.scanner.read().ok();
                    self.cur = LexerToken::make(TokenValue::RightParens)
                }
                // Equals
                b'=' => {
                    self.scanner.read()?;
                    self.scanner.expect_and_consume(b'=')?;
                    self.cur = LexerToken::make(TokenValue::Equals)
                }
                // Not equals
                b'!' => {
                    self.scanner.read()?;
                    self.scanner.expect_and_consume(b'=')?;
                    self.cur = LexerToken::make(TokenValue::NotEquals)
                }
                // Less, or equal
                b'<' => {
                    self.cur =
                        self.greater_or_less((TokenValue::LessThan, TokenValue::LessThanOrEqual))?;
                }
                // Greater or equal
                b'>' => {
                    self.cur = self.greater_or_less((
                        TokenValue::GreaterThan,
                        TokenValue::GreaterThanOrEqual,
                    ))?;
                }
                // Wildcard
                b'*' => {
                    self.scanner.read()?;
                    self.scanner.expect_and_consume_seq("==")?;
                    self.cur = LexerToken::make(TokenValue::WildcardEq)
                }
                // Ids
                b'a'..=b'z' => {
                    let id = parse_id(&mut self.scanner)?;
                    if self.scanner.is_eof {
                        self.cur = LexerToken::make_path(Path::from(id));
                    } else {
                        self.scanner.consume_white_spaces()?;
                        match self.scanner.cur {
                            b'?' => {
                                self.scanner.read().ok();

                                self.cur = LexerToken::make(TokenValue::Rel(Symbol::from(
                                    id.to_string().as_str(),
                                )))
                            }
                            b'-' => {
                                self.scanner.read()?;
                                if self.scanner.cur == b'>' {
                                    self.scanner.read()?;
                                    self.scanner.consume_white_spaces()?;
                                    self.cur = self.parse_path(id)?;
                                } else if self.scanner.is_lower() {
                                    self.cur = self.parse_rel(id)?;
                                } else {
                                    return self.scanner.make_generic_err(&format!(
                                        "Invalid symbol: '{}'",
                                        self.scanner.cur
                                    ));
                                }
                            }
                            _ => self.cur = LexerToken::make_path(Path::from(id)),
                        }
                    }
                }
                _ => {
                    return self
                        .scanner
                        .make_generic_err(&format!("Invalid symbol: '{}'", self.scanner.cur))
                }
            }
            return Ok(&self.cur);
        }
        self.cur = LexerToken::make_empty();
        Ok(&self.cur)
    }

    pub(super) fn make_generic_err<T>(&self, msg: &str) -> Result<T, Error> {
        self.scanner.make_generic_err(msg)
    }

    fn greater_or_less(&mut self, variants: (TokenValue, TokenValue)) -> Result<LexerToken, Error> {
        Ok(if let Some(char) = self.scanner.safe_peek() {
            if char == b'=' {
                self.scanner.read()?;
                self.scanner.read().ok();
                LexerToken::make(variants.1)
            } else {
                self.scanner.read().ok();
                LexerToken::make(variants.0)
            }
        } else {
            self.scanner.read().ok();
            LexerToken::make(variants.0)
        })
    }

    fn parse_rel(&mut self, id: Id) -> Result<LexerToken, Error> {
        let mut ids = Vec::from([id]);
        let mut complete = false;
        while !self.scanner.is_eof {
            let id = parse_id(&mut self.scanner)?;
            ids.push(id);
            if self.scanner.cur == b'-' {
                self.scanner.read()?;
                continue;
            } else if self.scanner.cur == b'?' {
                self.scanner.read().ok();
                complete = true;
                break;
            }
        }

        if complete {
            Ok(LexerToken::make(TokenValue::Rel(Symbol::from(
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join("-")
                    .as_str(),
            ))))
        } else {
            self.scanner.make_expect_err(b'?')
        }
    }

    fn parse_path(&mut self, id: Id) -> Result<LexerToken, Error> {
        let mut path = Vec::from([id]);
        while !self.scanner.is_eof {
            let id = parse_id(&mut self.scanner)?;
            path.push(id);
            self.scanner.consume_white_spaces()?;
            if self.scanner.cur == b'-' {
                self.scanner.read()?;
                self.scanner.expect_and_consume(b'>')?;
                self.scanner.consume_white_spaces()?;
                if !self.scanner.is_lower() {
                    return self.scanner.make_generic_err("Expecting path segment");
                }
            } else if !self.scanner.is_lower() {
                break;
            }
        }
        Ok(LexerToken::make(TokenValue::Path(Path::from(path))))
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

#[cfg(test)]
mod test {
    use super::super::path::Path;
    use super::TokenValue;
    use super::{Lexer, LexerToken};
    use crate::haystack::val::{Date, DateTime, Ref, Symbol, Time, Uri};
    use std::io::Cursor;
    use std::str::FromStr;

    #[test]
    fn test_filter_lexer_make() {
        let mut input = Cursor::new("".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(lexer.read().ok(), Some(&LexerToken::make_empty()));

        let mut input = Cursor::new(" \n\r\t ".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_empty()),
            "Expect empty token"
        );
    }

    #[test]
    fn test_filter_lexer_string() {
        let mut input = Cursor::new(r#""""#.as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value("".into())),
            "Expect Empty string"
        );

        let mut input = Cursor::new(r#""a \u2665 string""#.as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value("a ♥ string".into())),
            "Unicode escape"
        );

        let mut input = Cursor::new("\"\\t\\n\\\\\"".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value("\t\n\\".into())),
            "Special characters escape"
        );

        let mut input = Cursor::new("\"".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(lexer.read().ok(), None, "Expect Unterminated");
    }

    #[test]
    fn test_filter_lexer_uri() {
        let mut input = Cursor::new("``".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(Uri::from("").into())),
            "Empty uri"
        );

        let mut input = Cursor::new("`/a/b\\#\\&`".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(Uri::from(r#"/a/b\#&"#).into())),
            "Special characters escape"
        );

        let mut input = Cursor::new("`".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(lexer.read().ok(), None, "Expect Unterminated");
    }

    #[test]
    fn test_filter_lexer_ref() {
        let mut input = Cursor::new(
            "@abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-.~".as_bytes(),
        );
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                Ref::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-.~")
                    .into()
            )),
            "Ref"
        );

        let mut input = Cursor::new("@".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Ref parsing error");
    }

    #[test]
    fn test_filter_lexer_symbol() {
        let mut input = Cursor::new(
            "^abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-.~".as_bytes(),
        );
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                Symbol::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-.~")
                    .into()
            )),
            "Ref"
        );

        let mut input = Cursor::new("^".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Symbol parsing error");

        let mut input = Cursor::new("^ABC".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Symbol parsing error");
    }

    #[test]
    fn test_filter_lexer_number() {
        let mut input = Cursor::new("0".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(0.0.into())),
            "Number"
        );

        let mut input = Cursor::new("-8".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value((-8.0).into())),
            "Number"
        );

        let mut input = Cursor::new("-42.42".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value((-42.42).into())),
            "Number"
        );

        let mut input = Cursor::new("1e1".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(10.into())),
            "Number"
        );

        let mut input = Cursor::new("1E-1".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(0.1.into())),
            "Number"
        );

        let mut input = Cursor::new("1_1_1".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(111.into())),
            "Number"
        );

        let mut input = Cursor::new("-E123".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Number parsing error");

        let mut input = Cursor::new("-INF".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Number parsing error");
    }

    #[test]
    fn test_filter_lexer_date() {
        let mut input = Cursor::new("2021-08-05".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                Date::from_str("2021-08-05").expect("Date").into()
            )),
            "Expect Date"
        );

        let mut input = Cursor::new("2000-".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Date parsing error");
    }

    #[test]
    fn test_filter_lexer_time() {
        let mut input = Cursor::new("18:08:59.890".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                Time::from_str("18:08:59.890").expect("Date").into()
            )),
            "Expect Time"
        );

        let mut input = Cursor::new("18:09:00".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                Time::from_str("18:09:00").expect("Date").into()
            )),
            "Expect Time"
        );

        let mut input = Cursor::new("18:08:".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Time parsing error");
    }

    #[test]
    fn test_filter_lexer_datetime() {
        let mut input = Cursor::new("2021-08-06T17:05:00Z".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_value(
                DateTime::from_str("2021-08-06T17:05:00Z")
                    .expect("DateTime")
                    .into()
            )),
            "Expect DateTime"
        );

        let mut input = Cursor::new("2021-08-06T".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect DateTime parsing error");
    }

    #[test]
    fn test_filter_lexer_path() {
        let mut input = Cursor::new("foo_bar".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_path(Path::from("foo_bar"))),
            "Expect Path"
        );

        let mut input = Cursor::new("a->b -> c ->d".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make_path(Path::from(vec![
                "a".into(),
                "b".into(),
                "c".into(),
                "d".into()
            ]))),
            "Expect Path"
        );

        let mut input = Cursor::new("a->b-".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Path error");

        let mut input = Cursor::new("a->b->".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Path error");
    }

    #[test]
    fn test_filter_lexer_parens() {
        let mut input = Cursor::new(" (".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::LeftParens)),
            "Expect LeftParens"
        );

        let mut input = Cursor::new(")".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::RightParens)),
            "Expect RightParens"
        );
    }

    #[test]
    fn test_filter_lexer_equals() {
        let mut input = Cursor::new("==".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::Equals)),
            "Expect Equals"
        );
    }

    #[test]
    fn test_filter_lexer_not_equals() {
        let mut input = Cursor::new("!=".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::NotEquals)),
            "Expect NotEquals"
        );

        let mut input = Cursor::new("! =".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect NotEquals error");
    }

    #[test]
    fn test_filter_lexer_wildcard_eq() {
        let mut input = Cursor::new("*==".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::WildcardEq)),
            "Expect WildcardEq"
        );

        let mut input = Cursor::new("*= =".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect WildcardEq error");
    }

    #[test]
    fn test_filter_lexer_less() {
        let mut input = Cursor::new("<".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::LessThan)),
            "Expect LessThan"
        );

        let mut input = Cursor::new("<=".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::LessThanOrEqual)),
            "Expect LessThanOrEqual"
        );
    }

    #[test]
    fn test_filter_lexer_great() {
        let mut input = Cursor::new(">".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::GreaterThan)),
            "Expect GreaterThan"
        );

        let mut input = Cursor::new(">=".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::GreaterThanOrEqual)),
            "Expect GreaterThanOrEqual"
        );
    }

    #[test]
    fn test_filter_lexer_rel() {
        let mut input = Cursor::new("a?".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::Rel("a".into()))),
            "Expect Rel"
        );

        let mut input = Cursor::new("a-b-c?".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert_eq!(
            lexer.read().ok(),
            Some(&LexerToken::make(TokenValue::Rel("a-b-c".into()))),
            "Expect Rel"
        );

        let mut input = Cursor::new("a-b-?".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Rel error");

        let mut input = Cursor::new("a-b".as_bytes());
        let mut lexer = Lexer::make(&mut input).expect("Should create lexer");

        assert!(lexer.read().is_err(), "Expect Rel error");
    }

    #[test]
    fn test_filter_lexer_iterator() {
        let mut input = Cursor::new("a ==(@foo)!=12 x-y?*==>< <=12:12:56`foo`a -> b->c".as_bytes());
        let lexer = Lexer::make(&mut input).expect("Should create lexer");

        for el in lexer.into_iter().enumerate() {
            let (i, tk) = el;

            match i {
                0 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::Path("a".into()))),
                    "Expect Path"
                ),
                1 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::Equals)),
                    "Expect Equals"
                ),
                2 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::LeftParens)),
                    "Expect LeftParens"
                ),
                3 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make_value(Ref::from("foo").into())),
                    "Expect Ref"
                ),
                4 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::RightParens)),
                    "Expect RightParens"
                ),
                5 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::NotEquals)),
                    "Expect NotEquals"
                ),
                6 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make_value(12.into())),
                    "Expect Number"
                ),
                7 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::Rel("x-y".into()))),
                    "Expect Rel"
                ),
                8 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::WildcardEq)),
                    "Expect WildcardEq"
                ),
                9 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::GreaterThan)),
                    "Expect GreaterThan"
                ),
                10 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::LessThan)),
                    "Expect LessThan"
                ),
                11 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make(TokenValue::LessThanOrEqual)),
                    "Expect LessThanOrEqual"
                ),
                12 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make_value(
                        Time::from_str("12:12:56").expect("Time").into()
                    )),
                    "Expect LessThanOrEqual"
                ),
                13 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make_value(Uri::from("foo").into())),
                    "Expect LessThanOrEqual"
                ),
                14 => assert_eq!(
                    tk.ok(),
                    Some(LexerToken::make_path(
                        vec!["a".into(), "b".into(), "c".into()].into()
                    )),
                    "Expect Path"
                ),
                _ => panic!("Unhandled lexer iterator case"),
            }
        }
    }
}
