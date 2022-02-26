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

    assert!(Filter::try_from("containedBy? equip2").is_err());
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
            Value::default()
        } else if path.len() == 1 {
            match root.get(&path[0].to_string()) {
                Some(val) => val.clone(),
                None => Value::default(),
            }
        } else {
            let mut cur_val = Value::default();
            let mut cur_dict = root.clone();

            for (pos, segment) in path.iter().enumerate() {
                cur_val = match cur_dict.get(&segment.to_string()) {
                    Some(Value::Dict(dict)) => Value::Dict(dict.clone()),
                    Some(Value::Ref(id)) => {
                        if let Some(dict) = self.resolve_ref(id) {
                            Value::Dict(dict)
                        } else {
                            Value::Null
                        }
                    }
                    _ => Value::Null,
                };

                if cur_val.is_null() {
                    break;
                }

                if pos < path.len() {
                    cur_dict = match &cur_val {
                        Value::Dict(dict) => dict.clone(),
                        _ => continue,
                    };
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
