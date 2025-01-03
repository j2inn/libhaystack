// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter parser

use super::lexer::{Lexer, LexerToken, TokenValue};
use super::nodes::*;
use super::path::Path;
use crate::haystack::encoding::zinc::decode::scanner::Scanner;
use crate::val::{Symbol, Value};
use lazy_static::lazy_static;

use std::io::{Error, Read};

///
/// Parser for Haystack Filter
///
pub struct Parser<Lexer> {
    lexer: Lexer,
}

// Internal constant tokens to be used when comparing against current tokens
lazy_static! {
    static ref OR_TOKEN: LexerToken = LexerToken::make_path("or".into());
    static ref AND_TOKEN: LexerToken = LexerToken::make_path("and".into());
}

impl<'a, R: Read> Parser<Lexer<Scanner<'a, R>>> {
    pub(crate) fn make(input: &'a mut R) -> Result<Self, Error> {
        let lexer = Lexer::make(input)?;
        Ok(Self { lexer })
    }

    pub fn parse(&mut self) -> Result<Or, Error> {
        self.lexer.read()?;
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Or, Error> {
        let mut ands: Vec<And> = vec![self.parse_and()?];

        while self.lexer.cur == *OR_TOKEN {
            if self.lexer.is_eof() {
                return self.make_generic_err("Expecting 'and' expression.");
            }
            self.lexer.read()?;
            ands.push(self.parse_and()?);
        }
        Ok(Or { ands })
    }

    fn parse_and(&mut self) -> Result<And, Error> {
        let mut terms: Vec<Term> = vec![self.parse_term()?];

        while self.lexer.cur == *AND_TOKEN {
            if self.lexer.is_eof() {
                return self.make_generic_err("Expecting 'term' expression.");
            }
            self.lexer.read()?;
            terms.push(self.parse_term()?);
        }

        Ok(And { terms })
    }

    fn parse_term(&mut self) -> Result<Term, Error> {
        match self.lexer.cur.value.clone() {
            Some(token) => match &token {
                // Parens
                TokenValue::LeftParens => Ok(Term::Parens(self.parse_parens()?)),
                // Path Expressions
                TokenValue::Path(path) => {
                    // Not an expression
                    if path.to_string() == "not" {
                        Ok(Term::Missing(self.parse_not()?))
                    } else if let Ok(next_token) = self.lexer.read() {
                        if let Some(next_token) = next_token.value.clone() {
                            self.parse_cmp_or_wildcard_eq(&next_token, path)
                        } else {
                            Ok(Term::Has(Has { path: path.clone() }))
                        }
                    } else {
                        Ok(Term::Has(Has { path: path.clone() }))
                    }
                }
                // Is A
                TokenValue::Value(Value::Symbol(symbol)) => {
                    let term = Term::IsA(IsA {
                        symbol: symbol.clone(),
                    });
                    self.lexer.read().ok();
                    Ok(term)
                }
                // Relation Expressions
                TokenValue::Rel(rel) => {
                    let term = Term::Relation(self.parse_rel(rel)?);
                    Ok(term)
                }
                _ => self.make_generic_err(&format!("Unexpected term token: {token:?}.")),
            },
            None => self.make_generic_err("Unexpected empty term token."),
        }
    }

    fn parse_cmp_or_wildcard_eq(
        &mut self,
        next_token: &TokenValue,
        path: &Path,
    ) -> Result<Term, Error> {
        match next_token {
            TokenValue::Equals
            | TokenValue::NotEquals
            | TokenValue::LessThan
            | TokenValue::LessThanOrEqual
            | TokenValue::GreaterThan
            | TokenValue::GreaterThanOrEqual => {
                let term = Term::Cmp(self.parse_cmp(path, next_token)?);
                self.lexer.read().ok();
                Ok(term)
            }
            TokenValue::WildcardEq => {
                let term = Term::WildcardEq(self.parse_wildcard_eq(path)?);
                self.lexer.read().ok();
                Ok(term)
            }
            _ => Ok(Term::Has(Has { path: path.clone() })),
        }
    }

    fn parse_parens(&mut self) -> Result<Parens, Error> {
        self.lexer.read()?;
        let or = self.parse_or()?;
        if self.lexer.cur.value != Some(TokenValue::RightParens) {
            return self.make_generic_err("Expecting ')'.");
        }
        self.lexer.read().ok();
        Ok(Parens { or })
    }

    fn parse_not(&mut self) -> Result<Missing, Error> {
        match self.lexer.read()?.value.clone() {
            Some(token) => match token {
                TokenValue::Path(path) => {
                    self.lexer.read().ok();
                    Ok(Missing { path })
                }
                _ => self.make_generic_err(&format!("Unexpected 'not' operant token: {token:?}.")),
            },
            None => self.make_generic_err("Unexpected empty 'not' expression."),
        }
    }

    fn parse_cmp(&mut self, path: &Path, cmp: &TokenValue) -> Result<Cmp, Error> {
        match self.lexer.read()?.value.clone() {
            Some(token) => match token {
                TokenValue::Value(value) => Ok(Cmp {
                    path: path.clone(),
                    op: self.to_cmp_op(cmp)?,
                    value,
                }),
                TokenValue::Path(maybe_bool) => {
                    let str_val = maybe_bool.to_string();
                    if str_val == "true" || str_val == "false" {
                        Ok(Cmp {
                            path: path.clone(),
                            op: self.to_cmp_op(cmp)?,
                            value: if str_val == "true" {
                                Value::make_true()
                            } else {
                                Value::make_false()
                            },
                        })
                    } else {
                        self.make_generic_err(&format!(
                            "Unexpected 'cmp' operant token: {str_val}."
                        ))
                    }
                }
                _ => self.make_generic_err(&format!("Unexpected 'cmp' operant token: {token:?}.")),
            },
            None => self.make_generic_err("Unexpected empty 'cmp' expression."),
        }
    }

    fn parse_wildcard_eq(&mut self, id: &Path) -> Result<WildcardEq, Error> {
        match self.lexer.read()?.value.clone() {
            Some(token) => match token {
                TokenValue::Value(Value::Ref(ref_value)) => Ok(WildcardEq {
                    id: id.clone(),
                    ref_value,
                }),
                _ => self.make_generic_err(&format!(
                    "Wildcard equality accepts only Ref operands, got: {token:?}."
                )),
            },
            None => self.make_generic_err("Unexpected empty Wildcard expression."),
        }
    }

    fn parse_rel(&mut self, rel: &Symbol) -> Result<Relation, Error> {
        let first_token = self.lexer.read()?.value.clone();

        // The first token can be either a term...
        let rel_term = match first_token.clone() {
            Some(TokenValue::Value(Value::Symbol(rel_term))) => Some(rel_term.clone()),
            _ => None,
        };

        // ..or a ref.
        let ref_value = match first_token {
            Some(TokenValue::Value(Value::Ref(ref_value))) => {
                self.lexer.read().ok();
                Some(ref_value.clone())
            }
            _ => {
                // If the first token is a term, the second token could be a ref.
                if rel_term.is_some() {
                    match self.lexer.read()?.value.clone() {
                        Some(TokenValue::Value(Value::Ref(ref_value))) => {
                            self.lexer.read().ok();
                            Some(ref_value.clone())
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
        };

        Ok(Relation {
            rel: rel.clone(),
            rel_term,
            ref_value,
        })
    }

    fn to_cmp_op(&self, token: &TokenValue) -> Result<CmpOp, Error> {
        match token {
            TokenValue::Equals => Ok(CmpOp::Eq),
            TokenValue::NotEquals => Ok(CmpOp::NotEq),
            TokenValue::LessThan => Ok(CmpOp::LessThan),
            TokenValue::LessThanOrEqual => Ok(CmpOp::LessThanEq),
            TokenValue::GreaterThan => Ok(CmpOp::GreatThan),
            TokenValue::GreaterThanOrEqual => Ok(CmpOp::GreatThanEq),
            _ => self.make_generic_err(&format!("Unexpected 'cmp' token: {token:?}.")),
        }
    }

    fn make_generic_err<'b: 'a, T>(&'b self, msg: &str) -> Result<T, Error> {
        self.lexer.make_generic_err(msg)
    }
}

#[cfg(test)]
mod test {
    use super::super::nodes::*;
    use super::super::path::Path;
    use super::Parser;
    use std::io::Cursor;

    #[test]
    fn test_filter_parser_make() {
        let mut input = Cursor::new("".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        assert_eq!(parser.parse().ok(), None);
    }

    #[test]
    fn test_filter_parser_or() {
        let mut input = Cursor::new("a or b".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let or = parser.parse().expect("Or");
        assert_eq!(or.to_string(), "a or b");
    }

    #[test]
    fn test_filter_parser_and() {
        let mut input = Cursor::new("a and b".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let and = parser.parse().expect("And");
        assert_eq!(and.to_string(), "a and b");
    }

    #[test]
    fn test_filter_parser_not() {
        let mut input = Cursor::new("not foo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        assert_eq!(
            parser.parse().ok(),
            Some(Or {
                ands: vec![And {
                    terms: vec![Term::Missing(Missing {
                        path: Path::from("foo")
                    })]
                }]
            })
        );
    }

    #[test]
    fn test_filter_parser_has() {
        let mut input = Cursor::new("foo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let has = parser.parse().expect("Has");
        assert_eq!(has.to_string(), "foo");
    }

    #[test]
    fn test_filter_parser_eq() {
        let mut input = Cursor::new("a == 2021-08-10".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let eq = parser.parse().expect("Eq");
        assert_eq!(eq.to_string(), "a == 2021-08-10");
    }

    #[test]
    fn test_filter_parser_not_eq() {
        let mut input = Cursor::new("a != false".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let neq = parser.parse().expect("Not Eq");
        assert_eq!(neq.to_string(), "a != false");
    }

    #[test]
    fn test_filter_parser_less() {
        let mut input = Cursor::new("x < 42ft".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let less = parser.parse().expect("Less");
        assert_eq!(less.to_string(), "x < 42ft");
    }

    #[test]
    fn test_filter_parser_less_eq() {
        let mut input = Cursor::new("x <= 12:12:00".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let less_eq = parser.parse().expect("Less Eq");
        assert_eq!(less_eq.to_string(), "x <= 12:12:00");
    }

    #[test]
    fn test_filter_parser_great() {
        let mut input = Cursor::new("x > 1e1".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let great = parser.parse().expect("Great");
        assert_eq!(great.to_string(), "x > 10");
    }

    #[test]
    fn test_filter_parser_great_eq() {
        let mut input = Cursor::new("x >= 0".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let great_eq = parser.parse().expect("Great Eq");
        assert_eq!(great_eq.to_string(), "x >= 0");
    }

    #[test]
    fn test_filter_parser_wildcard_eq() {
        let mut input = Cursor::new("x *== @foo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let wildcard_eq = parser.parse().expect("Wildcard Eq");
        assert_eq!(wildcard_eq.to_string(), "x *== @foo");
    }

    #[test]
    fn test_filter_parser_parens() {
        let mut input = Cursor::new("a or (b and c)".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let parens = parser.parse().expect("Parens");
        assert_eq!(parens.to_string(), "a or ( b and c )");
    }

    #[test]
    fn test_filter_parser_rel_no_term_no_id() {
        let mut input = Cursor::new("foo?".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo?");

        let mut input = Cursor::new("foo? and other".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? and other");
    }

    #[test]
    fn test_filter_parser_rel_term_no_id() {
        let mut input = Cursor::new("foo? ^bar".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? ^bar");

        let mut input = Cursor::new("foo? ^bar and other".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? ^bar and other");
    }

    #[test]
    fn test_filter_parser_rel_term_id() {
        let mut input = Cursor::new("foo? ^bar @zoo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? ^bar @zoo");

        let mut input = Cursor::new("foo? ^bar @zoo and other".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? ^bar @zoo and other");
    }

    #[test]
    fn test_filter_parser_rel_no_term_id() {
        let mut input = Cursor::new("foo? @zoo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? @zoo");

        let mut input = Cursor::new("foo? @zoo and other".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let rel = parser.parse().expect("Relation");
        assert_eq!(rel.to_string(), "foo? @zoo and other");
    }

    #[test]
    fn test_filter_parser_isa() {
        let mut input = Cursor::new("^geoPlace and not campusRef".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");

        let symbol = parser.parse().expect("Symbol");
        assert_eq!(symbol.to_string(), "^geoPlace and not campusRef");

        let mut input = Cursor::new("^geoPlace and not".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");
        assert!(parser.parse().is_err(), "Expecting parse error.");

        let mut input = Cursor::new("not foo and bar and zoo".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Should create parser");
        let symbol = parser.parse().expect("Symbol");
        assert_eq!(
            symbol,
            Or {
                ands: vec![And {
                    terms: vec![
                        Term::Missing(Missing { path: "foo".into() }),
                        Term::Has(Has { path: "bar".into() }),
                        Term::Has(Has { path: "zoo".into() })
                    ]
                }]
            }
        );
    }
}
