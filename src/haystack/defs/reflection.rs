// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Def reflection

use std::collections::BTreeMap;

use super::namespace::Namespace;
use crate::haystack::defs::namespace::DefDict;
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
            Some(entity) => {
                let mut types_with_inheritance = BTreeMap::<&Dict, Vec<&Dict>>::new();

                for def in &self.defs {
                    let inheritance = self.ns.inheritance(def.def_symbol());
                    if inheritance.contains(&entity) {
                        types_with_inheritance.insert(def, inheritance.clone());
                    }
                }

                if types_with_inheritance.len() == 1 {
                    // Just get the first entity if only one has been found.
                    types_with_inheritance
                        .keys()
                        .copied()
                        .next()
                        .map_or(Dict::default(), |def| def.clone())
                } else {
                    // If multiple entity tags have been found then we need to find which tag is the most specific.
                    // This can happen if a record has a tag like `ahu` and `equip`. The `ahu` tag extends `equip`.
                    // Therefore, we need to check all the inheritance to find the first tag that isn't any of the
                    // other tag's inheritance. This tag should be the most specific entity.

                    let all_defs = types_with_inheritance.keys().copied();

                    all_defs
                        .into_iter()
                        .find(|def| {
                            // If this def isn't in the inheritance of any other tag then we should
                            // have the most specific entity.
                            !types_with_inheritance
                                .iter()
                                .any(|(inner_def, inheritance)| {
                                    inner_def != def && inheritance.contains(def)
                                })
                        })
                        .map_or(Dict::default(), |def| def.clone())
                }
            }
            None => Dict::default(),
        }
    }
}
