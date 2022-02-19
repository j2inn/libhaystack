// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter parsing and evaluation

//!
//! Haystack filters are used to evaluate simple expressions
//! against [Dict](crate::val::Dict) and [Filter](crate::filter::Filter) [Paths](crate::filter::path::Path).
//!
//! # Examples
//!
//! ```
//! // Find if a Dictionary contains a `dis` value equals to `Test`
//! use libhaystack::dict;
//! use libhaystack::val::*;
//! use libhaystack::filter::{Filter, Filtered};
//!
//! let filter = Filter::try_from(r#"site and dis=="Test""#).expect("Filter");
//! let dict = dict!{"site" => Value::make_marker(), "dis" => Value::from("Test")};
//!
//! assert_eq!(dict.filter(&filter), true);
//! ```

pub mod eval;
pub mod filtered;
mod lexer;
mod nodes;
pub mod parser;
pub mod path;
pub mod resolver;

// Export common types
pub use eval::Eval;
pub use filtered::{Filtered, ListFiltered};
pub use resolver::PathResolver;

use nodes::*;
use parser::Parser;
use std::io::Cursor;

use self::eval::EvalContext;

/// A Haystack Filter
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Filter {
    or: Or,
}

impl TryFrom<&str> for Filter {
    type Error = std::io::Error;
    /// Parse a Haystack Filter from `&str`
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let mut input = Cursor::new(data.as_bytes());
        let mut parser = Parser::make(&mut input)?;
        Ok(Self {
            or: parser.parse()?,
        })
    }
}

impl Eval for Filter {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        self.or.eval(context)
    }
}
