// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc List decoding

use super::super::parser::ParserType;
use crate::haystack::val::List;
use std::io::{Error, Read};

/// Parse a List
pub(crate) fn parse_list<'a, 'b: 'a, R: Read>(
    parser: &'a mut ParserType<'b, R>,
) -> Result<List, Error> {
    let mut list = List::new();

    parser.lexer.expect_char(b'[', "List parser")?;

    let mut expect_comma = false;
    let mut done = false;

    while !done {
        parser.lexer.read()?;

        if parser.lexer.is_char(b']') {
            done = true;
            break;
        }

        if expect_comma {
            parser.lexer.expect_char(b',', "List parser")?;
            expect_comma = !expect_comma;
            continue;
        }

        list.push(parser.parse_value()?);
        expect_comma = true;
        if parser.lexer.is_eof() {
            break;
        }
    }

    if !done {
        return parser.lexer.make_generic_err("Invalid list, missing ']'");
    }

    Ok(list)
}

#[cfg(test)]
mod test {
    use super::super::super::parser::Parser;
    use super::parse_list;
    use crate::haystack::val::Value;
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_list() {
        let mut input = Cursor::new("[1.0, 2.0, [M,R],]".as_bytes());
        let mut parser = Parser::make(&mut input).expect("Parser");

        let list = parse_list(&mut parser);
        assert_eq!(
            list.ok(),
            Some(vec![
                Value::make_int(1),
                Value::make_int(2),
                vec![Value::Marker, Value::Remove].into()
            ])
        )
    }
}
