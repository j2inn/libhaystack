// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack path resolvers

use super::path::Path;
use crate::{
    haystack::val::{Dict, Value},
    val::{List, Ref},
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
    /// Resolve a Path
    fn resolve(&self, path: &Path) -> Value;
    /// Resolve a Ref
    fn resolve_ref(&self, reference: &Ref) -> Option<Dict>;

    ///
    /// Resolves the refs in the provided list.
    ///
    /// # Arguments
    /// - ref_list The list of refs to resolve
    ///
    /// # Returns
    /// If no refs are found in the list, or the refs do not resolve,
    /// returns [Null Value](crate::val::Value::Null), otherwise returns the [Dict Value](crate::val::Value::Dict)
    ///
    ///
    fn resolve_ref_list(&self, ref_list: &List) -> Vec<Dict> {
        ref_list
            .iter()
            .filter_map(|val| {
                if let Value::Ref(id) = val {
                    self.resolve_ref(id)
                } else {
                    None
                }
            })
            .collect::<Vec<Dict>>()
    }
}

/// Simple resolver that resolves path indirections inside [Dict](crate::val::Dict)
impl PathResolver for Dict {
    fn resolve_for(&self, root: &Dict, path: &Path) -> Value {
        if path.is_empty() || root.is_empty() {
            Value::Null
        } else {
            let mut cur_val = root.clone().into();

            for segment in path.iter() {
                cur_val = match cur_val {
                    Value::Dict(dict) => dict
                        .get(&segment.to_string())
                        .map_or(Value::Null, |v| v.clone()),
                    _ => Value::Null,
                };

                if cur_val.is_null() {
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
