// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc identifier decoding

use super::scanner::Scanner;
use std::io::{Error, Read};
use std::string::ToString;

/// Zinc identifier
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Id {
    pub(super) value: String,
}

impl From<&str> for Id {
    //! Converts from `&str` to an `Id`
    fn from(value: &str) -> Self {
        Id {
            value: String::from(value),
        }
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

/// Parse an identifier
pub(crate) fn parse_id<R: Read>(scanner: &mut Scanner<R>) -> Result<Id, Error> {
    if !scanner.is_lower() {
        return scanner.make_generic_err("Ids have to start with a lower case letter");
    }
    let value = parse_literal(scanner)?;

    Ok(Id { value })
}

/// Parse a Zinc literal, such as `NA`
pub(super) fn parse_literal<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    let mut id = Vec::new();

    while !scanner.is_eof && (scanner.is_alpha_num() || scanner.is_any_of("_")) {
        id.push(scanner.cur);
        if let Err(err) = scanner.read() {
            if !scanner.is_eof {
                return Err(err);
            }
        }
    }

    if !id.is_empty() {
        Ok(String::from_utf8_lossy(&id).to_string())
    } else {
        scanner.make_generic_err("Unexpected empty literal")
    }
}

#[cfg(test)]
mod test {
    use super::{parse_id, parse_literal};
    use std::io::Cursor;

    #[test]
    fn test_zinc_parse_literal() {
        let mut input = Cursor::new("Id_to_test".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let literal = parse_literal(&mut scanner);
        assert_eq!(literal.ok(), Some("Id_to_test".into()))
    }

    #[test]
    fn test_zinc_parse_id() {
        let mut input = Cursor::new("fooBar".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let id = parse_id(&mut scanner);
        assert_eq!(id.ok(), Some("fooBar".into()))
    }

    #[test]
    fn test_zinc_parse_id_invalid() {
        let mut input = Cursor::new("ABC".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let literal = parse_id(&mut scanner);

        assert!(literal.is_err());
    }

    #[test]
    fn test_zinc_parse_literal_invalid() {
        let mut input = Cursor::new("*123".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let literal = parse_literal(&mut scanner);

        assert!(literal.is_err());
    }
}
