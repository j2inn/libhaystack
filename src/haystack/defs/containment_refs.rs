// Copyright (C) 2026 J2 Innovations. All Rights Reserved.

use crate::{
    defs::namespace::{DefDict, Namespace},
    filter::{Filter, FilterBuilder},
    val::{Dict, HaystackDict, Ref, Symbol, Value},
};

/// Options controlling containment ref lookups and population.
#[derive(Default)]
pub struct ContainmentRefOptions {
    /// When `true`:
    /// - Defs marked `deprecated` in the namespace are included in query results.
    /// - `floorRef` is also populated by [`add_containment_refs`] for compatibility
    ///   with older software that uses `floorRef` instead of `spaceRef`.
    pub deprecated: bool,
}

/// Returns the names of all ref defs that have a `containedBy` tag.
///
/// When `options.deprecated` is `false` (the default), defs that carry a
/// `deprecated` tag are excluded from the results.
pub fn get_containment_refs(
    namespace: &Namespace,
    options: Option<&ContainmentRefOptions>,
) -> Vec<String> {
    let default_opts = ContainmentRefOptions::default();
    let options = options.unwrap_or(&default_opts);

    namespace
        .all_subtypes_of(&Symbol::make("ref"))
        .iter()
        .filter(|def| def.has("containedBy"))
        .filter(|def| options.deprecated || !def.has("deprecated"))
        .map(|def| def.def_name().to_string())
        .collect()
}

/// Returns the names of all ref defs whose `containedBy` value is a subtype of `super_type`.
///
/// When `options.deprecated` is `false` (the default), defs that carry a
/// `deprecated` tag are excluded from the results.
pub fn get_contained_by_refs_for_super_type(
    namespace: &Namespace,
    super_type: &str,
    options: Option<&ContainmentRefOptions>,
) -> Vec<String> {
    let default_opts = ContainmentRefOptions::default();
    let options = options.unwrap_or(&default_opts);

    let super_sym = Symbol::make(super_type);
    namespace
        .all_subtypes_of(&Symbol::make("ref"))
        .iter()
        .filter(|def| def.has("containedBy"))
        .filter(|def| options.deprecated || !def.has("deprecated"))
        .filter_map(|def| {
            let contained_by_sym = def.get_symbol("containedBy")?;
            if namespace.fits(&Symbol::make(contained_by_sym.value.as_str()), &super_sym) {
                Some(def.def_name().to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Adds containment refs from `parent` to `dict` using namespace metadata.
///
/// Copies `siteRef`, `equipRef`, and `spaceRef` as appropriate, then finds the
/// best-fit containment ref for the parent entity type.
///
/// `options` may be `None` to use defaults. If `options.deprecated` is `true`, a
/// `floorRef` is also populated for compatibility with older software that uses
/// `floorRef` over `spaceRef`.
///
/// Returns the name of the containment ref that was applied (empty string if none).
pub fn add_containment_refs(
    dict: &mut Dict,
    parent: &Dict,
    namespace: &Namespace,
    options: Option<&ContainmentRefOptions>,
) -> String {
    let default_opts = ContainmentRefOptions::default();
    let options = options.unwrap_or(&default_opts);

    // siteRef
    let site_ref = if parent.has("site") {
        parent.get("id").cloned()
    } else {
        parent.get("siteRef").cloned()
    };
    if let Some(v) = site_ref {
        dict.insert("siteRef".into(), v);
    }

    // equipRef
    let equip_ref = if parent.has("equip") {
        parent.get("id").cloned()
    } else {
        parent.get("equipRef").cloned()
    };
    if let Some(v) = equip_ref {
        dict.insert("equipRef".into(), v);
    }

    // spaceRef
    let space_ref = if parent.has("space") || parent.has("floor") {
        parent.get("id").cloned()
    } else {
        parent.get("spaceRef").cloned()
    };
    if let Some(v) = space_ref {
        dict.insert("spaceRef".into(), v);
    }

    // floorRef (deprecated compatibility)
    if options.deprecated {
        let floor_ref = if parent.has("floor") {
            parent.get("id").cloned()
        } else {
            parent.get("floorRef").cloned()
        };
        if let Some(v) = floor_ref {
            dict.insert("floorRef".into(), v);
        }
    }

    // Determine the entity type of the parent via namespace reflection.
    let entity_type_name = namespace
        .def_of_dict(parent)
        .get_symbol("def")
        .map(|s| s.value.clone())
        .unwrap_or_default();

    if entity_type_name.is_empty() {
        return String::new();
    }

    let ref_name = find_containment_ref_for_type(namespace, &entity_type_name)
        .map(|def| def.def_name().to_string())
        .unwrap_or_default();

    if !ref_name.is_empty()
        && let Some(parent_id) = parent.get("id").cloned()
        && !dict.contains_key(&ref_name)
    {
        dict.insert(ref_name.clone(), parent_id);
    }

    ref_name
}

/// Finds the ref def whose `containedBy` type fits `entity_type_name`.
fn find_containment_ref_for_type(namespace: &Namespace, entity_type_name: &str) -> Option<Dict> {
    namespace
        .all_subtypes_of(&Symbol::make("ref"))
        .into_iter()
        .filter(|def| def.has("containedBy"))
        .find(|def| {
            def.get_symbol("containedBy")
                .map(|cb| namespace.fits(&Symbol::make(entity_type_name), cb))
                .unwrap_or(false)
        })
        .cloned()
}

/// Builds a Haystack equality filter for an id tag.
pub fn make_id_filter(r: &Ref) -> Filter {
    FilterBuilder::new()
        .eq("id", Value::make_ref(r.value.as_str()))
        .build()
}

/// Converts a Haystack value to a Ref if it is one.
pub fn value_as_ref(v: &Value) -> Option<&Ref> {
    match v {
        Value::Ref(r) => Some(r),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::namespace::Namespace;
    use crate::dict;
    use crate::val::{Dict, Ref, Value};

    fn test_namespace() -> Namespace {
        use crate::encoding::zinc::decode::from_str as zinc_decode;
        let zinc = include_str!("../../../tests/defs/defs.zinc");
        let value: Value = zinc_decode(zinc).unwrap_or_else(|e| panic!("valid defs zinc: {e:?}"));
        let grid =
            crate::val::Grid::try_from(&value).unwrap_or_else(|e| panic!("valid defs grid: {e:?}"));
        Namespace::make(grid)
    }

    #[test]
    fn get_containment_refs_non_empty() {
        let ns = test_namespace();
        let refs = get_containment_refs(&ns, None);
        assert!(
            !refs.is_empty(),
            "expected containment refs to be non-empty"
        );
    }

    #[test]
    fn get_contained_by_refs_for_equip() {
        let ns = test_namespace();
        let refs = get_contained_by_refs_for_super_type(&ns, "equip", None);
        assert!(
            refs.iter().all(|r| r.ends_with("Ref")),
            "expected all results to be ref names, got: {refs:?}"
        );
    }

    #[test]
    fn get_containment_refs_excludes_deprecated_by_default() {
        let ns = test_namespace();
        let without = get_containment_refs(&ns, None);
        let with_deprecated =
            get_containment_refs(&ns, Some(&ContainmentRefOptions { deprecated: true }));
        // With deprecated=true we get at least as many results; any extra must
        // be defs that carry a `deprecated` tag.
        assert!(
            with_deprecated.len() >= without.len(),
            "enabling deprecated should not reduce results"
        );
        // None of the default results should be deprecated defs.
        // (All returned names must not be marked deprecated in the namespace.)
        for name in &without {
            let sym = Symbol::make(name.as_str());
            if let Some(def) = ns.get(&sym) {
                assert!(
                    !def.has("deprecated"),
                    "default results must not contain deprecated def '{name}'"
                );
            }
        }
    }

    #[test]
    fn get_contained_by_refs_excludes_deprecated_by_default() {
        let ns = test_namespace();
        let without = get_contained_by_refs_for_super_type(&ns, "equip", None);
        let with_deprecated = get_contained_by_refs_for_super_type(
            &ns,
            "equip",
            Some(&ContainmentRefOptions { deprecated: true }),
        );
        assert!(
            with_deprecated.len() >= without.len(),
            "enabling deprecated should not reduce results"
        );
        for name in &without {
            let sym = Symbol::make(name.as_str());
            if let Some(def) = ns.get(&sym) {
                assert!(
                    !def.has("deprecated"),
                    "default results must not contain deprecated def '{name}'"
                );
            }
        }
    }

    #[test]
    fn make_id_filter_returns_correct_filter() {
        let r = Ref::make("abc123", None);
        let f = make_id_filter(&r);
        assert_eq!(f.to_string(), "id == @abc123");
    }

    #[test]
    fn value_as_ref_returns_some_for_ref_value() {
        let r = Ref::make("abc", None);
        let v = Value::Ref(r.clone());
        let result = value_as_ref(&v);
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, "abc");
    }

    #[test]
    fn value_as_ref_returns_none_for_non_ref() {
        let v = Value::make_str("hello");
        assert!(value_as_ref(&v).is_none());
    }

    #[test]
    fn add_containment_refs_copies_site_ref() {
        let ns = test_namespace();
        let parent = dict! {
            "id" => Value::make_ref("parent-id"),
            "siteRef" => Value::make_ref("site-id")
        };
        let mut child = Dict::new();
        add_containment_refs(&mut child, &parent, &ns, None);
        assert_eq!(
            child.get("siteRef").and_then(|v| if let Value::Ref(r) = v {
                Some(r.value.as_str())
            } else {
                None
            }),
            Some("site-id")
        );
    }

    #[test]
    fn add_containment_refs_site_entity_uses_id() {
        let ns = test_namespace();
        let parent = dict! {
            "id" => Value::make_ref("the-site"),
            "site" => Value::make_marker()
        };
        let mut child = Dict::new();
        add_containment_refs(&mut child, &parent, &ns, None);
        assert_eq!(
            child.get("siteRef").and_then(|v| if let Value::Ref(r) = v {
                Some(r.value.as_str())
            } else {
                None
            }),
            Some("the-site")
        );
    }

    #[test]
    fn add_containment_refs_deprecated_floor_ref() {
        let ns = test_namespace();
        let parent = dict! {
            "id" => Value::make_ref("floor-id"),
            "floor" => Value::make_marker()
        };
        let mut child_default = Dict::new();
        add_containment_refs(&mut child_default, &parent, &ns, None);
        assert!(
            child_default.get("floorRef").is_none(),
            "floorRef should not be set without deprecated option"
        );

        let opts = ContainmentRefOptions { deprecated: true };
        let mut child_deprecated = Dict::new();
        add_containment_refs(&mut child_deprecated, &parent, &ns, Some(&opts));
        assert_eq!(
            child_deprecated
                .get("floorRef")
                .and_then(|v| if let Value::Ref(r) = v {
                    Some(r.value.as_str())
                } else {
                    None
                }),
            Some("floor-id")
        );
    }
}
