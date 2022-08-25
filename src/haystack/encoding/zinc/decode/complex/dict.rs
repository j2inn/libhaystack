// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Dict decoding

use super::super::parser::ParserType;
use crate::haystack::val::{Dict, Value};
use std::io::{Error, Read};

/// Parse a Dict
pub(crate) fn parse_dict<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<Dict, Error> {
    parser.lexer.expect_char(b'{', "Dict parser")?;
    parser.lexer.read()?;

    let dict = parse_dict_parts(parser)?;

    parser.lexer.expect_char(b'}', "Dict parser")?;

    Ok(dict)
}

pub(super) fn parse_dict_parts<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<Dict, Error> {
    let mut dict = Dict::new();
    let mut expect_comma = false;

    while !parser.lexer.is_eof() {
        if expect_comma && parser.lexer.is_char(b',') {
            parser.lexer.read()?;
            expect_comma = false;
            continue;
        }

        if !parser.lexer.is_id() {
            break;
        }

        let key = &parser.lexer.expect_id()?;
        expect_comma = true;

        parser.lexer.read()?;

        if parser.lexer.is_eof() {
            dict.insert(key.to_string(), Value::Marker);
            break;
        }

        if parser.lexer.is_char(b':') {
            parser.lexer.read()?;
            let value = parser.parse_value()?;
            dict.insert(key.to_string(), value);
            parser.lexer.read()?;
        } else {
            dict.insert(key.to_string(), Value::Marker);
        }
    }

    Ok(dict)
}

#[cfg(test)]
mod test {
    use super::super::super::parser::Parser;
    use super::parse_dict;
    use crate::dict;
    use crate::haystack::val::*;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_dict() {
        let mut input = Cursor::new("{num: 100, site,bool:T}".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let dict = parse_dict(&mut parser);
        assert_eq!(
            dict.ok(),
            Some(
                dict! {"num"=> Value::make_int(100), "site" => Value::Marker, "bool" => Value::make_true()}
            )
        )
    }
}
