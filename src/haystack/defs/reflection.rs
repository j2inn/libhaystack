// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Def reflection

use super::namespace::Namespace;
use crate::haystack::val::{Dict, HaystackDict, Symbol};

/// The reflection result on a target dictionary
pub struct Reflection<'a> {
    /// The subject of the reflection operation
    pub subject: &'a Dict,
    /// Matching definitions for the target dictionary
    pub defs: Vec<Dict>,
    /// The def namespace used for the reflection
    pub ns: &'a Namespace,
    /// The entity type of the target dictionary
    pub entity_type: Dict,
}

impl<'a> Reflection<'a> {
    pub fn make(subject: &'a Dict, defs: Vec<Dict>, ns: &'a Namespace) -> Self {
        let mut reflect = Reflection {
            subject,
            defs,
            ns,
            entity_type: Dict::default(),
        };
        reflect.compute_entity_type();
        reflect
    }

    /// True if the refection matches the base definition
    pub fn fits(&self, base: &Symbol) -> bool {
        self.defs.iter().any(|def| {
            if let Some(def) = def.get_symbol("def") {
                self.ns.fits(def, base)
            } else {
                false
            }
        })
    }

    /// Compute the entity type of the target dictionary
    fn compute_entity_type(&mut self) {
        self.entity_type = match self.ns.get_by_name("entity") {
            Some(entity) => self
                .defs
                .iter()
                .find(|def| {
                    if let Some(def) = def.get_symbol("def") {
                        self.ns.inheritance(def).contains(entity)
                    } else {
                        false
                    }
                })
                .map_or(Dict::default(), |v| v.to_owned()),
            None => Dict::default(),
        }
    }
}
