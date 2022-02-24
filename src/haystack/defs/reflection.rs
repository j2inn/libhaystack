// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Def reflection

use super::namespace::Namespace;
use crate::haystack::val::{Dict, HaystackDict, Symbol};

/// The reflection result on a target dictionary
pub struct Reflection<'a> {
    /// The subject of the reflection operation
    pub subject: Dict,
    /// Matching definitions for the target dictionary
    pub defs: Vec<&'a Dict>,
    /// The def namespace used for the reflection
    pub ns: &'a Namespace<'a>,
    /// The entity type of the target dictionary
    pub entity_type: Dict,
}

impl<'a> Reflection<'a> {
    pub fn make(subject: &Dict, defs: Vec<&'a Dict>, ns: &'a Namespace<'a>) -> Self {
        let mut reflect = Reflection {
            subject: subject.clone(),
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
                        self.ns.inheritance(def).contains(&entity)
                    } else {
                        false
                    }
                })
                .map_or(Dict::default(), |v| (*v).clone()),
            None => Dict::default(),
        }
    }
}
