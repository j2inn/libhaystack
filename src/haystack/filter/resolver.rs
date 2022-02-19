// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack path resolvers

use super::path::Path;
use crate::{
    haystack::val::{Dict, Value},
    val::Ref,
};

/// Generic resolver of a `Path` for a given type
///
/// The resolver job is to dereferences `Path` elements in a `Filter`
///
/// Implementations will resolve each path segment starting from tags in the provided
/// `Dict` until path segments are completely traversed, or a [Null Value](crate::val::Value::Null) is resolved
pub trait PathResolver {
    /// Specify a Dict root to resolve against
    fn resolve_for(&self, root: &Dict, path: &Path) -> Value;
    /// Resolve a Path using implementation specific root path
    fn resolve(&self, path: &Path) -> Value;
    /// Resolve a Ref based on the root Dict
    fn resolve_ref(&self, reference: &Ref) -> Option<Dict>;
}

/// Simple resolver that resolves path indirections inside [Dict](crate::val::Dict)
impl PathResolver for Dict {
    fn resolve_for(&self, root: &Dict, path: &Path) -> Value {
        if path.is_empty() || root.is_empty() {
            Value::default()
        } else {
            let mut cur_dict: &Dict = root;
            let mut cur_val = Value::default();
            for segment in path.iter() {
                if let Some(val) = cur_dict.get(&segment.to_string()) {
                    if let Value::Dict(dict) = val {
                        cur_dict = dict;
                    }
                    cur_val = val.clone();
                } else {
                    break;
                }
            }
            cur_val
        }
    }

    fn resolve(&self, path: &Path) -> Value {
        self.resolve_for(self, path)
    }

    fn resolve_ref(&self, _reference: &Ref) -> Option<Dict> {
        None
    }
}
