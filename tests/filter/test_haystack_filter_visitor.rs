// Copyright (C) 2020 - 2022, J2 Innovations

//! Tests the Haystack Filter

use libhaystack::filter::nodes::{
    And, Cmp, Has, IsA, Missing, Or, Parens, Relation, Visitable, Visitor, WildcardEq,
};

use libhaystack::filter::Filter;

#[derive(PartialEq, Debug)]
struct CountVisitor {
    or_count: u32,
    and_count: u32,
    parens_count: u32,
    has_count: u32,
    missing_count: u32,
    is_a_count: u32,
    wildcard_equals_count: u32,
    relation_count: u32,
    cmp_count: u32,
}

impl CountVisitor {
    fn new() -> Self {
        CountVisitor {
            or_count: 0,
            and_count: 0,
            parens_count: 0,
            has_count: 0,
            missing_count: 0,
            is_a_count: 0,
            wildcard_equals_count: 0,
            relation_count: 0,
            cmp_count: 0,
        }
    }
}

impl Visitor for CountVisitor {
    fn visit_cond_or(&mut self, node: &Or) {
        self.or_count += 1;
        node.ands.iter().for_each(|and| and.accept_visitor(self));
    }

    fn visit_cond_and(&mut self, node: &And) {
        self.and_count += 1;
        node.terms.iter().for_each(|term| term.accept_visitor(self));
    }

    fn visit_parens(&mut self, node: &Parens) {
        self.parens_count += 1;
        node.or.accept_visitor(self);
    }

    fn visit_has(&mut self, _node: &Has) {
        self.has_count += 1;
    }

    fn visit_missing(&mut self, _node: &Missing) {
        self.missing_count += 1;
    }

    fn visit_is_a(&mut self, _node: &IsA) {
        self.is_a_count += 1;
    }

    fn visit_wildcard_equals(&mut self, _node: &WildcardEq) {
        self.wildcard_equals_count += 1;
    }

    fn visit_relation(&mut self, _node: &Relation) {
        self.relation_count += 1;
    }

    fn visit_cmp(&mut self, _node: &Cmp) {
        self.cmp_count += 1;
    }
}

#[test]
fn test_filter_count_nodes() {
    let filter_str = "site and foo or goo and not true and (test == true and 
        test != false and test > 34.52 and test 
        < -1 and (test == 1.34342) and test <= 1 and test == \"some string\" and test 
        == @foo and test == `/foo` and test->foo < 1)";

    let filter = Filter::try_from(filter_str).unwrap();
    let mut visitor = CountVisitor::new();
    filter.accept_visitor(&mut visitor);

    assert_eq!(
        visitor,
        CountVisitor {
            or_count: 3,
            and_count: 4,
            parens_count: 2,
            has_count: 3,
            missing_count: 1,
            is_a_count: 0,
            wildcard_equals_count: 0,
            relation_count: 0,
            cmp_count: 10,
        }
    );
}
