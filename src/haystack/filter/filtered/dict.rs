// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Filtered Dict

use super::super::eval::Eval;
use super::super::eval::EvalContext;
use super::super::Filter;
use super::Filtered;
use crate::defs::namespace::Namespace;
use crate::val::Dict;

/// Implement the filtering trait for a Dict
/// # Example
/// ```
///  use libhaystack::dict;
///  use libhaystack::val::{Dict, Value};
///  use libhaystack::filter::{Filter, Filtered};
///  let dict = dict!{
///     "site" => Value::make_marker(),
///     "dis" => Value::make_str("Some site")
///  };
///  assert!(dict.filter(&Filter::try_from("site and dis").expect("Filter")));
/// ```
impl Filtered<'_, bool> for Dict {
    fn filter(&self, filter: &Filter) -> bool {
        let ns = Namespace::default();
        let context = EvalContext::make(self, &ns, self);
        filter.eval(&context)
    }
}
