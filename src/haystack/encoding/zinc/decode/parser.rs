// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc parser implementation

use super::complex::dict::parse_dict;
use super::complex::grid::parse_grid;
use super::complex::list::parse_list;
use super::lexer::{Lexer, TokenValue};
use super::scanner::Scanner;
use crate::haystack::val::Value;
use std::io::{Error, Read};

/// Generic parser type that composes the lexer and scanner types
pub type ParserType<'a, R> = Parser<Lexer<Scanner<'a, R>>>;

pub struct Parser<Lexer> {
    pub(super) lexer: Lexer,
}

impl<'a, 'b: 'a, R: Read> Parser<Lexer<Scanner<'b, R>>> {
    /// Constructs a [Parser](self::Parser) for the provided [Read](std::io::Read)
    pub fn make(input: &'b mut R) -> Result<Self, Error> {
        let mut lexer = Lexer::make(input)?;
        // Advance lexer to first token
        lexer.read()?;
        Ok(Parser { lexer })
    }

    /// Parses a Haystack [Value](crate::val::Value) form the provided [Read](std::io::Read)
    /// stream.
    pub fn parse_value(&mut self) -> Result<Value, Error> {
        match &self.lexer.cur.value {
            Some(value) => match value {
                // Possible Grid ver
                TokenValue::Id(_) => {
                    let grid = parse_grid(self)?;
                    Ok(Value::make_grid(grid))
                }
                // Scalar Value
                TokenValue::Value(value) => Ok(value.clone()),
                // Complex values
                TokenValue::ZincChar(char) => match char {
                    // List Start
                    b'[' => {
                        let list = parse_list(self)?;
                        Ok(Value::make_list(list))
                    }
                    // Dict Start
                    b'{' => {
                        let dict = parse_dict(self)?;
                        Ok(Value::make_dict(dict))
                    }
                    // Nested Grid Start
                    b'<' => {
                        let grid = parse_grid(self)?;
                        Ok(Value::make_grid(grid))
                    }
                    _ => self.lexer.make_generic_err(&format!(
                        "Unexpected parser token '{token}'",
                        token = *char as char
                    )),
                },
            },
            None => Ok(Value::Null),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::haystack::val::Value;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parser_ok() {
        let mut input = Cursor::new("^foo");
        let parser = Parser::make(&mut input);

        assert!(parser.is_ok());

        assert_eq!(
            parser.unwrap().parse_value().ok(),
            Some(Value::make_symbol("foo"))
        );
    }

    #[test]
    fn test_zinc_parser_err() {
        let mut input = Cursor::new("ver:@foo");
        let parser = Parser::make(&mut input);

        assert!(parser.is_ok());

        assert_eq!(
            parser
                .unwrap()
                .parse_value()
                .map_err(|err| err.to_string())
                .err(),
            Some("Expecting 'ver' to be a Str, got 'Ref(Ref { value: \"foo\", dis: None })'. Input position: 7[0], line 1".into())
        );
    }
}
