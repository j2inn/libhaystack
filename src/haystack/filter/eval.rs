// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter evaluation

use super::path::Path;
use super::resolver::PathResolver;
use crate::{defs::namespace::Namespace, haystack::val::Dict, val::Value};

/// Evaluate a `Filter` expression to `true` or `false`
pub trait Eval {
    /// Generic eval function that can use the `PathResolver` trait
    /// for resolving paths specified in the `Filter`
    ///
    /// # Arguments
    /// * `dict` - the Dict to match the Filter against
    /// * `resolver` - a generic `PathResolver` that resolves `Path`s
    /// indirection specified in the `Filter`
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool;
}

/// Filter Eval context.
/// Captures the required elements for a filter to be evaluated.
///
pub struct EvalContext<'a, 'b: 'a, R: PathResolver> {
    pub dict: &'a Dict,
    pub ns: &'b Namespace<'b>,
    pub resolver: &'a R,
}

impl<'a, 'b: 'a, R: PathResolver> EvalContext<'a, 'b, R> {
    /// Make a FilterEval context
    ///
    /// # Arguments
    /// * `dict` - the Dict to match the Filter against
    /// * `ns` - the Def namespace to match symbol definitions against
    /// * `resolver` - a generic `PathResolver` that resolves `Path`s
    /// indirection specified in the `Filter`
    pub fn make(dict: &'a Dict, ns: &'b Namespace<'b>, resolver: &'a R) -> Self {
        Self { dict, ns, resolver }
    }

    /// Resolves the given path against the dict and the context
    pub fn resolve_for_dict(&self, dict: &Dict, path: &Path) -> Value {
        self.resolver.resolve_for(dict, path)
    }

    /// Resolves the given path against the context
    pub fn resolve(&self, path: &Path) -> Value {
        self.resolver.resolve_for(self.dict, path)
    }
}
