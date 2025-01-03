// Copyright (C) 2020 - 2022, J2 Innovations

//! Tests the Haystack Filter with defs
#[cfg(test)]
use crate::defs::utils::parse_def;
use lazy_static::lazy_static;
use libhaystack::defs::namespace::Namespace;
use libhaystack::dict;
use libhaystack::filter::eval::{Eval, EvalContext};
use libhaystack::filter::path::Path;
use libhaystack::filter::{Filter, PathResolver};
use libhaystack::haystack::val::*;

lazy_static! {
    pub(super) static ref NS: Namespace<'static> = Namespace::make(parse_def());
}

fn eval_in_context(filter: &Filter, dict: &Dict) -> bool {
    let cx = EvalContext::make(&dict, &NS, dict);
    filter.eval(&cx)
}

#[test]
fn test_filter_defs_isa() {
    let dict = dict! {"site"=> Value::make_marker()};

    let filter = Filter::try_from("^site").expect("Filter");
    assert!(eval_in_context(&filter, &dict));

    let filter = Filter::try_from("^geoPlace").expect("Filter");
    assert!(eval_in_context(&filter, &dict));

    let filter = Filter::try_from("^ahu").expect("Filter");
    assert!(!eval_in_context(&filter, &dict));
}

#[test]
fn test_filter_defs_containment() {
    let recs = vec![
        dict! {
            "id" => Value::make_ref("site"),
            "site" => Value::Marker,
            "dis" => "site".into(),
        },
        dict! {
            "id" => Value::make_ref("equip1"),
            "equip" => Value::Marker,
            "navName" => "equip1".into(),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("equip2"),
            "equip" => Value::Marker,
            "navName" => "equip2".into(),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("point1"),
            "point" => Value::Marker,
            "navName" => "point1".into(),
            "equipRef" => Value::make_ref("equip1"),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("point2"),
            "point" => Value::Marker,
            "navName" => "point2".into(),
            "equipRef" => Value::make_ref("equip1"),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("point3"),
            "point" => Value::Marker,
            "navName" => "point3".into(),
            "equipRef" => Value::make_ref("equip2"),
            "siteRef" => Value::make_ref("site"),
        },
    ];

    let db = Records { recs };

    let filter = Filter::try_from("containedBy? @site").expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(1));

    let filter = Filter::try_from("containedBy? @site").expect("Filter");
    assert_eq!(db.read_all(&filter).len(), 5);

    let filter = Filter::try_from("containedBy? @site and point").expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));

    let filter = Filter::try_from("containedBy? @equip1 and point").expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));

    let filter = Filter::try_from("containedBy? @equip1").expect("Filter");
    assert_eq!(db.read_all(&filter).len(), 2);

    let filter = Filter::try_from("containedBy? @point").expect("Filter");
    assert_eq!(db.read(&filter), None);

    let filter = Filter::try_from("containedBy? @equip2").expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(5));
}

#[test]
fn test_filter_path() {
    let recs = vec![
        dict! {
            "id" => Value::make_ref("site"),
            "site" => Value::Marker,
            "dis" => "site".into(),
        },
        dict! {
            "id" => Value::make_ref("equip1"),
            "equip" => Value::Marker,
            "navName" => "equip1".into(),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("equip2"),
            "equip" => Value::Marker,
            "navName" => "equip2".into(),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("point1"),
            "point" => Value::Marker,
            "navName" => "point1".into(),
            "equipRef" => Value::make_ref("equip1"),
            "siteRef" => Value::make_ref("site"),
        },
    ];

    let db = Records { recs };

    let filter = Filter::try_from(r#"point and equipRef->siteRef->foo=="site""#).expect("Filter");
    assert_eq!(db.read(&filter), None);

    let filter = Filter::try_from(r#"point and equipRef->siteRef->dis=="site""#).expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));
}

#[test]
fn test_filter_ref_list_path() {
    let recs = vec![
        dict! {
            "id" => Value::make_ref("site"),
            "site" => Value::Marker,
            "dis" => "site".into(),
        },
        dict! {
            "id" => Value::make_ref("equip1"),
            "equip" => Value::Marker,
            "navName" => "equip1".into(),
            "siteRef" => Value::make_ref("site"),
        },
        dict! {
            "id" => Value::make_ref("equip2"),
            "equip" => Value::Marker,
            "navName" => "equip2".into(),
            "siteRef" => Value::make_ref("site"),
            "refList2" => Value::make_list(vec![Value::make_ref("equip1")])
        },
        dict! {
            "id" => Value::make_ref("point1"),
            "point" => Value::Marker,
            "navName" => "point1".into(),
            "equipRef" => Value::make_ref("equip1"),
            "siteRef" => Value::make_ref("site"),
            "refList" => Value::make_list(vec![Value::make_ref("equip1"), Value::make_ref("equip2")])
        },
    ];

    let db = Records { recs };

    let filter = Filter::try_from(r#"refList->siteRef->foo=="site""#).expect("Filter");
    assert_eq!(db.read(&filter), None);

    let filter = Filter::try_from(r#"refList->navName=="equip1""#).expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));

    let filter = Filter::try_from(r#"refList->navName=="equip2""#).expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));

    let filter = Filter::try_from(r#"refList->refList2->siteRef->dis=="site""#).expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));

    let filter = Filter::try_from(r#"refList==@equip2"#).expect("Filter");
    assert_eq!(db.read(&filter), db.recs.get(3));
}

struct Records {
    recs: Vec<Dict>,
}

impl Records {
    fn read(&self, filter: &Filter) -> Option<&Dict> {
        self.recs.iter().find(|rec| {
            let cx = EvalContext::make(&rec, &NS, self);
            filter.eval(&cx)
        })
    }

    fn read_all(&self, filter: &Filter) -> Vec<&Dict> {
        self.recs
            .iter()
            .filter_map(|rec| {
                let cx = EvalContext::make(&rec, &NS, self);
                if filter.eval(&cx) {
                    Some(rec)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl PathResolver for Records {
    fn resolve_for(&self, root: &Dict, path: &Path) -> Value {
        if path.is_empty() || root.is_empty() {
            Value::Null
        } else if path.len() == 1 {
            root.get(&path[0].to_string())
                .map_or(Value::Null, |v| v.clone())
        } else {
            let mut cur_val = Value::Dict(root.clone());

            for (pos, segment) in path.iter().enumerate() {
                let seg_str = &segment.to_string();

                cur_val = match &cur_val {
                    Value::Dict(dict) => dict.get(seg_str).map_or(Value::Null, |v| v.clone()),

                    Value::Ref(id) => self.resolve_ref(id).map_or(Value::Null, |dict| {
                        dict.get(seg_str).map_or(Value::Null, |v| v.clone())
                    }),

                    Value::List(list) => {
                        let dicts = self.resolve_ref_list(list);

                        if dicts.is_empty() {
                            Value::Null
                        } else {
                            let next_path: Path =
                                path.iter().skip(pos).cloned().collect::<Vec<_>>().into();

                            let vals: Vec<Value> = dicts
                                .iter()
                                .map(|dict| self.resolve_for(dict, &next_path))
                                .filter(|v| v.has_value())
                                .collect();

                            if vals.is_empty() {
                                Value::Null
                            } else {
                                return Value::List(vals);
                            }
                        }
                    }

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
        for segment in path.iter() {
            self.recs.iter().find(|r| r.has(&segment.to_string()));
        }

        Value::Null
    }

    fn resolve_ref(&self, id: &Ref) -> Option<Dict> {
        self.recs
            .iter()
            .find(|rec| rec.get_ref("id") == Some(id))
            .cloned()
    }
}
