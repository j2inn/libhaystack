// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Def namespace

use dashmap::{mapref::one::Ref as MapReadRef, DashMap};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashSet};

use super::misc::parse_multi_line_string_to_dicts;
use super::reflection::Reflection;
use crate::val::{Dict, Grid, HaystackDict, Ref, Symbol, Value};

/// Type of the whole defs database
pub type Defs = BTreeMap<Symbol, Dict>;
/// Type of all defs associated with a symbol
pub type SymbolDefs = BTreeMap<Symbol, Vec<Dict>>;

lazy_static! {
    pub(super) static ref EMPTY_SYMBOL: Symbol = Symbol::from("");
    pub(super) static ref EMPTY_DICT: Dict = Dict::default();
    pub(super) static ref EMPTY_VEC_DICT: Vec<Dict> = Vec::default();
    pub static ref DEFAULT_NS: Namespace<'static> = Namespace::default();
}

/// Def trait for a Haystack [Dict](crate::val::Dict)
pub trait DefDict: HaystackDict {
    /// Return the `def` [Symbol](crate::val::Symbol), empty symbol if there is no `def` tag, for the def dict
    fn def_symbol(&self) -> &Symbol {
        self.get_symbol("def").unwrap_or(&EMPTY_SYMBOL)
    }

    /// Return the `def` [Symbol](crate::val::Symbol) name
    fn def_name(&self) -> &String {
        &self.def_symbol().value
    }
}

impl DefDict for Dict {}

/// Definitions for the Haystack Core [Value](crate::val::Value) Types
pub struct CoreTypeDefs<'a> {
    pub marker: &'a Dict,
    pub na: &'a Dict,
    pub bool: &'a Dict,
    pub number: &'a Dict,
    pub coord: &'a Dict,
    pub str: &'a Dict,
    pub symbol: &'a Dict,
    pub reference: &'a Dict,
    pub uri: &'a Dict,
    pub xstr: &'a Dict,
    pub date: &'a Dict,
    pub time: &'a Dict,
    pub datetime: &'a Dict,
    pub dict: &'a Dict,
    pub list: &'a Dict,
    pub grid: &'a Dict,
}

/// The container of the normalized definitions.
#[derive(Debug, Default)]
pub struct Namespace<'a> {
    /// Collection of normalized defs.
    pub defs: Defs,

    /// Symbol to subtype defs.
    pub subtypes: SymbolDefs,
    /// Symbols to subtypes for all defs that are choices.
    pub choices: SymbolDefs,

    /// A list of all available conjunct defs.
    pub conjuncts: Vec<Dict>,
    pub conjuncts_keys: BTreeMap<String, Vec<Vec<String>>>,
    /// A list of feature defs.
    pub features: Vec<Dict>,
    /// A list of all the libs implemented by this namespace.
    pub libs: Vec<Dict>,
    // List of the features names.
    pub feature_names: Vec<String>,
    /// A list of all the `tagOn` names.
    pub tag_on_names: Vec<String>,
    /// A object that maps def names to their respective `tagOn` defs.
    pub tag_on_defs: SymbolDefs,

    // Lazy caching for super types
    supertypes_of_cache: DashMap<Symbol, Vec<&'a Dict>>,
    // Lazy caching for inheritance
    inheritance_of_cache: DashMap<Symbol, Vec<&'a Dict>>,
}

impl<'a> Namespace<'a> {
    /// Constructs a new namespace
    /// # Arguments
    /// - defs The normalized defs grid
    pub fn make(defs: Grid) -> Self {
        let mut ns = Namespace {
            defs: defs
                .into_iter()
                .filter_map(|rec| rec.get_symbol("def").map(|def| (def.clone(), rec.clone())))
                .collect(),

            ..Default::default()
        };

        // Order of execution matters as calls depend on the previous calculation
        ns.compute_subtypes();
        ns.compute_choices();

        ns.compute_conjuncts();
        ns.compute_conjuncts_keys();

        ns.compute_features();
        ns.compute_libs();

        ns.compute_feature_names();
        ns.compute_tag_on_names();
        ns.compute_tag_on_defs();

        ns
    }

    /// Return a def via its symbol if it can't be found.
    pub fn get(&self, symbol: &Symbol) -> Option<&Dict> {
        self.defs.get(symbol)
    }

    /// Return a def via its name if it can't be found.
    pub fn get_by_name(&self, name: &str) -> Option<&Dict> {
        self.defs.get(&Symbol::from(name))
    }

    /// Return true if the def name exists in the namespace.
    pub fn has_name(&self, name: &str) -> bool {
        self.defs.contains_key(&Symbol::from(name))
    }

    /// Return true if the def exists in the namespace.
    pub fn has(&self, symbol: &Symbol) -> bool {
        self.defs.contains_key(symbol)
    }

    /// Return a list of defs matching the names.
    pub fn all_matching_names(&self, names: &[&str]) -> Vec<&Dict> {
        names
            .iter()
            .filter_map(|name| self.get_by_name(name))
            .collect()
    }

    /// Compute a list of all available conjunct defs.
    fn compute_conjuncts(&mut self) {
        self.conjuncts.extend(
            self.defs
                .iter()
                .filter_map(|(sym, dict)| {
                    if Namespace::is_conjunct(sym) {
                        Some(dict)
                    } else {
                        None
                    }
                })
                .cloned(),
        );
    }

    /// True if the name is for a conjunct.
    pub fn is_conjunct(symbol: &Symbol) -> bool {
        symbol.value.contains('-')
    }

    /// Decomposes a conjunct into its respective defs and returns them
    pub fn conjuncts_defs(&self, symbol: &Symbol) -> Vec<&Dict> {
        self.all_matching_names(&symbol.value.split('-').collect::<Vec<&str>>())
    }

    /// Computes a list of feature defs.
    fn compute_features(&mut self) {
        self.features.extend(
            self.defs
                .iter()
                .filter_map(|(sym, dict)| {
                    if Namespace::is_feature(sym) {
                        Some(dict)
                    } else {
                        None
                    }
                })
                .cloned(),
        )
    }

    /// True if the name is for a feature.
    pub fn is_feature(symbol: &Symbol) -> bool {
        symbol.value.contains(':')
    }

    /// Computes a list of all the libs implemented by this namespace.
    fn compute_libs(&mut self) {
        self.libs
            .extend(self.subtypes_of(&Symbol::from("lib")).clone())
    }

    /// Returns the subtypes of the type.
    pub fn subtypes_of(&self, symbol: &Symbol) -> &Vec<Dict> {
        self.subtypes.get(symbol).unwrap_or(&EMPTY_VEC_DICT)
    }

    /// True if the def has subtypes.
    pub fn has_subtype(&self, symbol: &Symbol) -> bool {
        match self.subtypes.get(symbol) {
            Some(vec) => !vec.is_empty(),
            None => false,
        }
    }

    /// Returns a flattened list of all the subtypes.
    /// # Arguments
    /// - symbol The def to get the subtypes of
    /// # Returns
    /// All subtypes
    pub fn all_subtypes_of(&self, symbol: &Symbol) -> Vec<&Dict> {
        let mut sub_types: HashSet<&Dict> = HashSet::new();

        let mut defs_stack = vec![self.subtypes_of(symbol)];

        while !defs_stack.is_empty() {
            if let Some(defs) = defs_stack.pop() {
                for def in defs {
                    sub_types.insert(def);

                    let next_subtypes = self.subtypes_of(def.def_symbol());

                    defs_stack.push(next_subtypes)
                }
            }
        }

        sub_types.into_iter().collect()
    }

    /// Computes an object with name to subtype defs
    fn compute_subtypes(&mut self) {
        for def in self.defs.values() {
            if let Some(is_a_list) = def.get_list("is") {
                for name in is_a_list {
                    match name {
                        Value::Symbol(sym) => self
                            .subtypes
                            .entry(sym.clone())
                            .or_default()
                            .push(def.clone()),
                        _ => continue,
                    }
                }
            }
        }
    }

    ///  Returns the supertypes of a def or an empty list if it can't be found.
    pub fn supertypes_of(&'a self, symbol: &Symbol) -> MapReadRef<'a, Symbol, Vec<&'a Dict>> {
        if let Some(super_types) = self.supertypes_of_cache.get(symbol) {
            super_types
        } else {
            let val = match self.get(symbol) {
                Some(def) => {
                    if let Some(is_a_list) = def.get_list("is") {
                        let mut defs: Vec<&Dict> = Vec::new();
                        for name in is_a_list {
                            match name {
                                Value::Symbol(sym) => {
                                    if let Some(dict) = self.get(sym) {
                                        defs.push(dict);
                                    } else {
                                        continue;
                                    }
                                }
                                _ => continue,
                            }
                        }
                        defs
                    } else {
                        Vec::default()
                    }
                }
                None => Vec::default(),
            };

            if !self.supertypes_of_cache.contains_key(symbol) {
                self.supertypes_of_cache.insert(symbol.clone(), val);
            }
            self.supertypes_of_cache.get(symbol).expect("Cached value")
        }
    }

    /// Returns a flattened list of all the supertypes in the whole supertype chain.
    pub fn all_supertypes_of(&'a self, symbol: &Symbol) -> Vec<&'a Dict> {
        let mut super_types: HashSet<&Dict> = HashSet::new();
        let mut defs_stack = vec![self.supertypes_of(symbol).clone()];

        while !defs_stack.is_empty() {
            if let Some(defs) = defs_stack.pop() {
                for def in defs {
                    super_types.insert(def);

                    let next_subtypes = self.supertypes_of(def.def_symbol());
                    if !next_subtypes.is_empty() {
                        defs_stack.push(next_subtypes.clone())
                    }
                }
            }
        }

        super_types.into_iter().collect()
    }

    /// Returns a list of choices for def.
    pub fn choices_for(&self, symbol: &Symbol) -> &Vec<Dict> {
        if self.get(symbol).is_some_and(|def| self.is_choice(def)) {
            self.subtypes_of(symbol)
        } else {
            &EMPTY_VEC_DICT
        }
    }

    /// Computes an object containing symbols to subtypes for
    /// all defs that are choices.
    fn compute_choices(&mut self) {
        for (sym, def) in &self.defs {
            // A choice extends directly from 'choice'.
            if self.is_choice(def) {
                self.choices
                    .insert(sym.clone(), self.choices_for(sym).clone());
            }
        }
    }

    fn is_choice(&self, def: &Dict) -> bool {
        def.get_list("is")
            .into_iter()
            .flatten()
            .any(|v| v == &Value::make_symbol("choice"))
    }

    /// Compute a list of the features names.
    fn compute_feature_names(&mut self) {
        let mut features = HashSet::<&str>::new();
        for sym in self.defs.keys() {
            if Namespace::is_feature(sym) {
                if let Some((first, _second)) = sym.value.split_once(':') {
                    features.insert(first);
                }
            }
        }
        self.feature_names
            .extend(features.into_iter().map(|v| v.into()))
    }

    /// Computes a list of all the `tagOn` names.
    fn compute_tag_on_names(&mut self) {
        let mut tag_ons = HashSet::<&str>::new();

        for def in self.defs.values() {
            if let Some(tag_on) = def.get_list("tagOn") {
                let names = tag_on.iter().filter_map(|v| match v {
                    Value::Symbol(sym) => Some(sym.value.as_str()),
                    _ => None,
                });

                tag_ons.extend(names);
            }
        }

        self.tag_on_names
            .extend(tag_ons.into_iter().map(|v| v.into()));
    }

    /// Computes a object that maps def names to their respective `tagOn` defs.
    fn compute_tag_on_defs(&mut self) {
        for (sym, def) in &self.defs {
            if let Some(tag_on) = def.get_list("tagOn") {
                let defs: Vec<Dict> = tag_on
                    .iter()
                    .filter_map(|v| match v {
                        Value::Symbol(sym) => self.get(sym),
                        _ => None,
                    })
                    .cloned()
                    .collect();
                if !def.is_empty() {
                    self.tag_on_defs.insert(sym.clone(), defs);
                }
            }
        }
    }

    /// Return the defs inheritance as a flattened array of defs.
    pub fn inheritance(&'a self, symbol: &Symbol) -> MapReadRef<'a, Symbol, Vec<&'a Dict>> {
        if let Some(inheritance) = self.inheritance_of_cache.get(symbol) {
            inheritance
        } else {
            let val = if let Some(def) = self.get(symbol) {
                let mut supertypes = HashSet::<&Dict>::new();
                supertypes.insert(def);
                supertypes.extend(self.all_supertypes_of(symbol));
                supertypes.into_iter().collect()
            } else {
                Vec::default()
            };
            if !self.inheritance_of_cache.contains_key(symbol) {
                self.inheritance_of_cache.insert(symbol.clone(), val);
            }
            self.inheritance_of_cache.get(symbol).expect("Cached value")
        }
    }

    ///Return a list of defs for the given association on the parent.
    /// # Arguments
    /// - parent The parent def.
    /// - association The association.
    ///
    pub fn associations(&'a self, parent: &Symbol, association: &Symbol) -> Vec<&'a Dict> {
        if let Some(association_def) = self.get(association) {
            // Make sure the association exists and is an association.
            if matches!(
                association_def
                    .get_list("is")
                    .map(|list| list.contains(&Value::make_symbol("association"))),
                None | Some(false)
            ) {
                return Vec::default();
            }

            // If the association isn't computed then just get the associated defs.
            // For instance, this will return here if the association is 'tagOn'.
            if !association_def.has("computedFromReciprocal") {
                return self
                    .get(parent)
                    .and_then(|def| def.get_list(&association.value))
                    .unwrap_or(&Vec::default())
                    .iter()
                    .filter_map(|value| match value {
                        Value::Symbol(sym) => self.get(sym),
                        _ => None,
                    })
                    .collect();
            }

            // Find the reciprocal def.
            if let Some(reciprocal_of) = association_def.get_symbol("reciprocalOf") {
                if self.get(reciprocal_of).is_some() {
                    // If searching for a computed association (i.e. `tags`) then more work is required.
                    // Search for all tagOns and match against the parent's inheritance.
                    return self.find_reciprocal_associations(parent, reciprocal_of);
                }
            }

            Vec::default()
        } else {
            Vec::default()
        }
    }

    /// Return the associations for the parent for the reciprocal association.
    fn find_reciprocal_associations(
        &'a self,
        parent: &Symbol,
        reciprocal_of: &Symbol,
    ) -> Vec<&'a Dict> {
        let inheritance = self.inheritance(parent);

        let mut matches = HashSet::<&Dict>::new();

        for def in self.defs.values() {
            if let Some(Value::List(list)) = def.get(&reciprocal_of.value) {
                list.iter()
                    .filter_map(|value| match value {
                        Value::Symbol(sym) => self.get(sym),
                        _ => None,
                    })
                    .filter(|target| inheritance.contains(target))
                    .for_each(|_| {
                        matches.insert(def);
                    });
            }
        }

        matches.into_iter().collect()
    }

    /// Return a vector of defs for the `is` association on the parent.
    /// # Arguments
    /// - parent The parent def
    /// # Return
    /// The `is` association defs
    pub fn is(&'a self, parent: &Symbol) -> Vec<&'a Dict> {
        self.associations(parent, &Symbol::from("is"))
    }

    /// Return a vector of defs for the `tagOn` association on the parent.
    /// # Arguments
    /// - parent The parent def
    /// # Return
    /// The `tagOn` association defs
    pub fn tag_on(&'a self, parent: &Symbol) -> Vec<&'a Dict> {
        self.associations(parent, &Symbol::from("tagOn"))
    }

    /// Return a vector of defs for the `tags` association on the parent.
    /// # Arguments
    /// - parent The parent def
    /// # Return
    /// The `tags` association defs
    pub fn tags(&'a self, parent: &Symbol) -> Vec<&'a Dict> {
        self.associations(parent, &Symbol::from("tags"))
    }

    /// Return the defs implemented by the subject dict.
    /// # Arguments
    /// - subject The subject to reflect
    /// # Return
    /// The reflected defs
    pub fn reflect<'b>(&'a self, subject: &'b Dict) -> Reflection<'a> {
        let mut defs = Vec::<&Dict>::new();
        let mut markers = HashSet::<&str>::new();

        // Get all defs for the subject tags
        for key in subject.keys() {
            if let Some(def) = self.get(&Symbol::from(key.as_str())) {
                defs.push(def);
                if subject.has_marker(key.as_str()) {
                    markers.insert(key);
                }
            }
        }

        // If the tag maps to a possible conjunct, then check if the dict
        // has all the conjunct's tags.
        defs.extend(self.find_conjuncts(&markers));

        // Infer the inheritance from all defs reflected from the previous steps.
        let reflected = self.find_supertypes_from_defs(defs);

        Reflection::make(subject, reflected, self)
    }

    fn compute_conjuncts_keys(&mut self) {
        self.conjuncts_keys.extend(
            self.conjuncts
                .iter()
                .map(|conjunct| conjunct.def_name())
                .map(|def_name| def_name.split('-').collect::<Vec<&str>>())
                .filter(|parts| !parts.is_empty())
                .fold(
                    BTreeMap::default(),
                    |mut map: BTreeMap<String, Vec<Vec<String>>>, parts| {
                        let marker = parts[0];
                        map.entry(marker.into()).or_default().push(
                            Vec::from(&parts[1..])
                                .into_iter()
                                .map(|v| v.into())
                                .collect(),
                        );
                        map
                    },
                ),
        )
    }

    fn find_conjuncts(&self, markers: &HashSet<&str>) -> Vec<&Dict> {
        let conjunct_map = &self.conjuncts_keys;
        let mut conjuncts = Vec::<&Dict>::new();

        for marker in markers {
            conjunct_map
                .get(&marker.to_string())
                .map_or(&Vec::default(), |v| v)
                .iter()
                .for_each(|parts| {
                    if parts.iter().all(|part| markers.contains(part.as_str())) {
                        let mut name = vec![marker.to_string()];
                        name.extend(parts.iter().cloned());
                        if let Some(def) = self.get_by_name(&name.join("-")) {
                            conjuncts.push(def)
                        }
                    }
                })
        }
        conjuncts
    }

    fn find_supertypes_from_defs<'b: 'a>(&'a self, defs: Vec<&'b Dict>) -> Vec<&'a Dict> {
        let mut reflected = HashSet::<&Dict>::new();

        for def in defs {
            reflected.insert(def);
            reflected.extend(self.all_supertypes_of(def.def_symbol()));
        }

        reflected.into_iter().collect()
    }

    pub fn def_of_dict<'b: 'a>(&'a self, subject: &'b Dict) -> Dict {
        self.reflect(subject).entity_type
    }

    /// Return true if the specified def `fits` the base def.
    ///
    /// If true this means that `def` is assignable to types of `base_def`.
    /// This is effectively the same as checking if `inheritance(def)` contains
    /// base.
    ///
    /// # Arguments
    /// - def The symbol to check
    /// - def_base The base definition
    /// # Returns
    ///  True if the def fits the base def.
    pub fn fits(&'a self, def: &Symbol, base_def: &Symbol) -> bool {
        if let Some(base) = self.get(base_def) {
            self.inheritance(def).contains(&base)
        } else {
            false
        }
    }

    /// Return true if the specified def is a marker.
    ///
    /// # Arguments
    /// - def The symbol to check
    /// # Returns
    /// True if the def is a marker.
    pub fn fits_marker(&'a self, def: &Symbol) -> bool {
        self.fits(def, &Symbol::from("marker"))
    }

    /// Return true if the specified def is a val.
    ///
    /// # Arguments
    /// - def The symbol to check
    /// # Returns
    /// True if the def is a value.
    pub fn fits_val(&'a self, def: &Symbol) -> bool {
        self.fits(def, &Symbol::from("val"))
    }

    /// Return true if the specified def is a choice.
    ///
    /// # Arguments
    /// - def The symbol to check
    /// # Returns
    /// True if the def is a choice.
    pub fn fits_choice(&'a self, def: &Symbol) -> bool {
        self.fits(def, &Symbol::from("choice"))
    }

    /// Return true if the specified def is a entity.
    ///
    /// # Arguments
    /// - def The symbol to check
    /// # Returns
    /// True if the def is a entity.
    pub fn fits_entity(&'a self, def: &Symbol) -> bool {
        self.fits(def, &Symbol::from("entity"))
    }

    /// Return the tags that should be added for implementation.
    ///
    ///  # Arguments
    /// - name The def name.
    /// # Returns
    ///  An array of defs to be added.
    ///
    pub fn implementation(&'a self, def: &Symbol) -> Vec<&'a Dict> {
        // 1.a Based on the tag name get the single def tag name.
        // 1.b If this is a conjunct get each tag from it.
        let conjuncts_defs = self.conjuncts_defs(def);

        // 1.c Feature keys are never implemented.
        let mut defs: Vec<&Dict> = conjuncts_defs
            .into_iter()
            .filter(|def| !Namespace::is_feature(def.def_symbol()))
            .collect();

        // 2. We walk the supertype tree of the def and apply any tag which is marked as mandatory.
        let mut super_types = HashSet::<&Dict>::new();

        for def in &defs {
            super_types.extend(self.all_supertypes_of(def.def_symbol()));
        }

        for super_type in super_types {
            if super_type.has_marker("mandatory") {
                defs.push(super_type)
            }
        }

        defs
    }

    /// Return a reflected list of children prototypes for the parent dict.
    ///
    /// # Arguments
    /// - parent The parent dict.
    /// # Returns
    /// A list of children.
    ///
    pub fn protos(&'a self, parent: &Dict) -> Vec<Dict> {
        parent
            .keys()
            .flat_map(|name| self.protos_from_def(parent, name))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Return a reflected list of children prototypes for the parent dict.
    ///
    /// # Arguments
    /// - parent The parent dict.
    /// # Returns
    /// A list of children.
    ///
    fn protos_from_def(&'a self, parent: &Dict, name: &str) -> Vec<Dict> {
        if let Some(def) = self.get_by_name(name) {
            if let Some(children) = def.get("children") {
                // Parse the children into a list of dicts.
                let protos: Vec<Dict> = match children {
                    Value::Str(str) => parse_multi_line_string_to_dicts(str),
                    Value::List(list) => list
                        .iter()
                        .filter_map(|val| match val {
                            Value::Dict(dict) => Some(dict.clone()),
                            _ => None,
                        })
                        .collect(),
                    _ => return Vec::default(),
                };

                // Find any flattened values.
                let flattened = self.find_flattened_children(def, parent);

                // Merge the flattened children.
                let protos: Vec<Dict> = protos
                    .into_iter()
                    .map(|mut dict| {
                        for (key, val) in flattened.iter() {
                            dict.insert(key.clone(), val.clone());
                        }
                        dict
                    })
                    .collect();
                return protos;
            }
        }

        Vec::default()
    }

    /// Find the flattened children on the parent dict.
    ///
    /// # Arguments
    /// - def The def that may have the `childrenFlatten` tag.
    /// - parent The parent to search for values.
    /// # Returns
    /// A dict with the flattened children information.
    fn find_flattened_children(&'a self, def: &Dict, parent: &Dict) -> Dict {
        def.get_list("childrenFlatten")
            .unwrap_or(&Vec::default())
            .iter()
            .filter_map(|val| match val {
                Value::Symbol(sym) => Some(sym),
                _ => None,
            })
            .fold(Dict::new(), |mut dict, symbol| {
                for key in parent.keys() {
                    if self.fits(&Symbol::from(key.as_str()), symbol) {
                        if let Some(value) = parent.get(key) {
                            if !value.is_null() {
                                dict.insert(key.to_string(), value.clone());
                            }
                        }
                    }
                }
                dict
            })
    }

    /// The defs for all of the core haystack value types.
    pub fn core_type_defs(&self) -> CoreTypeDefs {
        CoreTypeDefs {
            marker: self.get_by_name("marker").unwrap_or(&EMPTY_DICT),
            na: self.get_by_name("na").unwrap_or(&EMPTY_DICT),
            bool: self.get_by_name("bool").unwrap_or(&EMPTY_DICT),
            number: self.get_by_name("number").unwrap_or(&EMPTY_DICT),
            coord: self.get_by_name("coord").unwrap_or(&EMPTY_DICT),
            str: self.get_by_name("str").unwrap_or(&EMPTY_DICT),
            symbol: self.get_by_name("symbol").unwrap_or(&EMPTY_DICT),
            reference: self.get_by_name("ref").unwrap_or(&EMPTY_DICT),
            uri: self.get_by_name("uri").unwrap_or(&EMPTY_DICT),
            xstr: self.get_by_name("xstr").unwrap_or(&EMPTY_DICT),
            date: self.get_by_name("date").unwrap_or(&EMPTY_DICT),
            time: self.get_by_name("time").unwrap_or(&EMPTY_DICT),
            datetime: self.get_by_name("dateTime").unwrap_or(&EMPTY_DICT),
            dict: self.get_by_name("dict").unwrap_or(&EMPTY_DICT),
            list: self.get_by_name("list").unwrap_or(&EMPTY_DICT),
            grid: self.get_by_name("grid").unwrap_or(&EMPTY_DICT),
        }
    }

    ///
    /// Query a subject's relationship.
    ///
    /// Relationships model how entities are related to one another via instance to instance
    /// relationships versus def to def associations.
    ///
    /// <https://project-haystack.dev/doc/docHaystack/Relationships#querying>
    ///
    /// # Arguments
    /// - subject The subject dict being queried.
    /// - rel_name The name of the relationship to query.
    /// - rel_term An optional relationship term to query against.
    /// - target_ref An optional reference target.
    /// - resolve An optional function that can resolve dicts (records) from a ref.
    /// # Returns
    ///  True if a match is made.
    ////
    pub fn has_relationship<F: Fn(&Ref) -> Option<Dict>>(
        &'a self,
        subject: &Dict,
        rel_name: &Symbol,
        rel_term: &Option<Symbol>,
        ref_target: &Option<Ref>,
        resolve: &F,
    ) -> bool {
        // Def must be registered.
        let Some(relationship) = self.get(rel_name) else {
            return false;
        };

        // Def must be a relationship.
        if !self
            .inheritance(rel_name)
            .iter()
            .any(|def| def.def_name() == "relationship")
        {
            return false;
        }

        let transitive = relationship.has_marker("transitive");

        // https://project-haystack.dev/doc/docHaystack/Relationships#reciprocalOf
        let reciprocal_of = relationship.get_symbol("reciprocalOf");

        let mut queried_refs = HashSet::<Ref>::new();
        let mut ref_tag: Option<Ref> = ref_target.as_ref().cloned();
        let mut subjects = vec![subject.clone()];

        'search: loop {
            let Some(cur_subject) = subjects.pop() else {
                break;
            };

            let id = cur_subject.get_ref("id");
            for (subject_key, subject_val) in cur_subject.iter() {
                let subject_def = self.get_by_name(subject_key);
                let mut rel_val = subject_def.and_then(|def| def.get(&rel_name.value));

                // Handle a reciprocal relationship. A reciprocal relationship can only
                // be inverted when a ref is specified.
                if rel_val.is_none() && ref_tag.as_ref() == id && subject_val.is_ref() {
                    if let Some(reciprocal_of) = reciprocal_of {
                        rel_val = subject_def.and_then(|def| def.get(&reciprocal_of.value));

                        if rel_val.is_some() {
                            if let Value::Ref(val) = subject_val {
                                ref_tag = Some(val.clone())
                            }
                        }
                    }
                }

                // Test to see if the relationship exists on any of the
                // reflected defs for an entry in subject.
                if let Some(Value::Symbol(rel_val)) = rel_val {
                    // If we're testing against a relationship value then
                    // ensure the target is also a symbol so we can see if it fits.
                    let mut has_match = if let Some(rel_term) = rel_term.as_ref() {
                        self.fits(rel_val, rel_term)
                    } else {
                        true
                    };

                    // Test to see if the value matches.
                    if has_match && ref_tag.is_some() {
                        has_match = false;

                        if matches!(subject_val, Value::Ref(val) if Some(val) == ref_tag.as_ref()) {
                            has_match = true;
                        } else if transitive {
                            if let Value::Ref(subject_val) = subject_val {
                                if !queried_refs.contains(subject_val) {
                                    queried_refs.insert(subject_val.clone());

                                    // If the value doesn't match but the relationship is transitive
                                    // then follow the refs until we find a match or not.
                                    // https://project-haystack.dev/doc/docHaystack/Relationships#transitive
                                    if let Some(new_subject) = resolve(subject_val) {
                                        if !new_subject.is_empty() {
                                            subjects.push(cur_subject);
                                            subjects.push(new_subject);
                                            continue 'search;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if has_match {
                        return true;
                    }
                }
            }
        }

        false
    }
}
