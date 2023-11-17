// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter node types

use super::eval::{Eval, EvalContext};
use super::path::Path;
use super::resolver::PathResolver;
use crate::haystack::val::{Ref, Value};
use crate::val::Symbol;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

/// A visitable that accepts a visitor.
pub trait Visitable {
    fn accept_visitor(&self, visitor: &mut impl Visitor);
}

/// A visitor trait used for working with a generated filter AST.
pub trait Visitor {
    fn visit_cond_or(&mut self, node: &Or);

    fn visit_cond_and(&mut self, node: &And);

    fn visit_parens(&mut self, node: &Parens);

    fn visit_has(&mut self, node: &Has);

    fn visit_missing(&mut self, node: &Missing);

    fn visit_is_a(&mut self, node: &IsA);

    fn visit_wildcard_equals(&mut self, node: &WildcardEq);

    fn visit_relation(&mut self, node: &Relation);

    fn visit_cmp(&mut self, node: &Cmp);
}

/// A Haystack Filter Or expression
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Or {
    pub ands: Vec<And>,
}

impl Display for Or {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.ands
            .iter()
            .enumerate()
            .try_for_each(|(idx, and)| -> Result {
                and.fmt(f)?;
                if idx < self.ands.len() - 1 {
                    f.write_str(" or ")
                } else {
                    Ok(())
                }
            })
    }
}

impl Eval for Or {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        self.ands.iter().any(|and| and.eval(context))
    }
}

impl Visitable for Or {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_cond_or(self);
    }
}

/// A Haystack Filter And expression
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct And {
    pub terms: Vec<Term>,
}

impl Eval for And {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        self.terms.iter().all(|term| term.eval(context))
    }
}

impl Display for And {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.terms
            .iter()
            .enumerate()
            .try_for_each(|(idx, term)| -> Result {
                term.fmt(f)?;
                if idx < self.terms.len() - 1 {
                    f.write_str(" and ")
                } else {
                    Ok(())
                }
            })
    }
}

impl Visitable for And {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_cond_and(self);
    }
}

/// A Haystack Filter terminal
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Term {
    Parens(Parens),
    Has(Has),
    Missing(Missing),
    IsA(IsA),
    WildcardEq(WildcardEq),
    Relation(Relation),
    Cmp(Cmp),
}

impl Eval for Term {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        match self {
            Term::Parens(parens) => parens.eval(context),
            Term::Has(has) => has.eval(context),
            Term::Missing(missing) => missing.eval(context),
            Term::IsA(isa) => isa.eval(context),
            Term::WildcardEq(wildcard_eq) => wildcard_eq.eval(context),
            Term::Relation(rel) => rel.eval(context),
            Term::Cmp(cmp) => cmp.eval(context),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Term::Parens(parens) => parens.fmt(f),
            Term::Has(has) => has.fmt(f),
            Term::Missing(missing) => missing.fmt(f),
            Term::IsA(isa) => isa.fmt(f),
            Term::WildcardEq(wildcard_eq) => wildcard_eq.fmt(f),
            Term::Relation(rel) => rel.fmt(f),
            Term::Cmp(cmp) => cmp.fmt(f),
        }
    }
}

impl Visitable for Term {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        match self {
            Term::Parens(parens) => parens.accept_visitor(visitor),
            Term::Has(has) => has.accept_visitor(visitor),
            Term::Missing(missing) => missing.accept_visitor(visitor),
            Term::IsA(isa) => isa.accept_visitor(visitor),
            Term::WildcardEq(wildcard_eq) => wildcard_eq.accept_visitor(visitor),
            Term::Relation(rel) => rel.accept_visitor(visitor),
            Term::Cmp(cmp) => cmp.accept_visitor(visitor),
        }
    }
}

/// Filter compare operators
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub enum CmpOp {
    Eq,
    NotEq,
    LessThan,
    LessThanEq,
    GreatThan,
    GreatThanEq,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Cmp {
    pub path: Path,
    pub op: CmpOp,
    pub value: Value,
}
impl Eval for Cmp {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        match self.op {
            CmpOp::Eq => cmp_dispatch(&PartialEq::eq, &context.resolve(&self.path), &self.value),
            CmpOp::NotEq => cmp_dispatch(&PartialEq::ne, &context.resolve(&self.path), &self.value),
            CmpOp::LessThan => {
                cmp_dispatch(&PartialOrd::lt, &context.resolve(&self.path), &self.value)
            }
            CmpOp::LessThanEq => {
                cmp_dispatch(&PartialOrd::le, &context.resolve(&self.path), &self.value)
            }
            CmpOp::GreatThan => {
                cmp_dispatch(&PartialOrd::gt, &context.resolve(&self.path), &self.value)
            }
            CmpOp::GreatThanEq => {
                cmp_dispatch(&PartialOrd::ge, &context.resolve(&self.path), &self.value)
            }
        }
    }
}

fn cmp_dispatch<Cmp: Fn(&Value, &Value) -> bool>(cmp: &Cmp, lhs: &Value, rhs: &Value) -> bool {
    match lhs {
        Value::List(list) => {
            if !rhs.is_list() {
                list.iter().any(|el| cmp_dispatch(cmp, el, rhs))
            } else {
                cmp(lhs, rhs)
            }
        }

        _ => cmp(lhs, rhs),
    }
}

impl Display for Cmp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.op {
            CmpOp::Eq => write!(f, "{} == {}", self.path, self.value),
            CmpOp::NotEq => write!(f, "{} != {}", self.path, self.value),
            CmpOp::LessThan => write!(f, "{} < {}", self.path, self.value),
            CmpOp::LessThanEq => write!(f, "{} <= {}", self.path, self.value),
            CmpOp::GreatThan => write!(f, "{} > {}", self.path, self.value),
            CmpOp::GreatThanEq => write!(f, "{} >= {}", self.path, self.value),
        }
    }
}

impl Visitable for Cmp {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_cmp(self);
    }
}

/// Missing term
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Missing {
    pub path: Path,
}
impl Eval for Missing {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        context.resolve(&self.path).is_null()
    }
}

impl Display for Missing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("not ")?;
        self.path.fmt(f)
    }
}

impl Visitable for Missing {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_missing(self);
    }
}

/// Parenthesis term
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Parens {
    pub or: Or,
}
impl Eval for Parens {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        self.or.eval(context)
    }
}

impl Display for Parens {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("( ")?;
        self.or.fmt(f)?;
        f.write_str(" )")
    }
}

impl Visitable for Parens {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_parens(self);
    }
}

/// Has term
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Has {
    pub path: Path,
}
impl Eval for Has {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        context.resolve(&self.path).has_value()
    }
}

impl Display for Has {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.path.fmt(f)
    }
}

impl Visitable for Has {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_has(self);
    }
}

/// Is A term
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct IsA {
    pub symbol: Symbol,
}
impl Eval for IsA {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        context.ns.reflect(context.dict).fits(&self.symbol)
    }
}

impl Display for IsA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("^")?;
        f.write_str(&self.symbol.value)
    }
}

impl Visitable for IsA {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_is_a(self);
    }
}

/// Wildcard equality
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct WildcardEq {
    pub id: Path,
    pub ref_value: Ref,
}
impl Eval for WildcardEq {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        // Stop circular dependencies.
        let mut resolved_refs = HashSet::<Ref>::new();

        // Keep resolving until we find the reference we're looking.
        let mut resolved_value = context.resolve(&self.id);

        let mut matched = false;

        while let Value::Ref(cur_ref) = &resolved_value {
            if cur_ref == &self.ref_value {
                matched = true;
                break;
            }
            if resolved_refs.contains(cur_ref) {
                break;
            }

            resolved_refs.insert(cur_ref.clone());

            let dict = context.resolver.resolve_ref(cur_ref);

            if let Some(dict) = dict {
                if !dict.is_empty() {
                    resolved_value = context.resolve_for_dict(&dict, &self.id);
                }
            } else {
                break;
            }
        }

        matched
    }
}

impl Display for WildcardEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} *== {}", &self.id, &self.ref_value)
    }
}

impl Visitable for WildcardEq {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_wildcard_equals(self);
    }
}

/// Relation
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Relation {
    pub rel: Symbol,
    pub ref_value: Option<Ref>,
}
impl Eval for Relation {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        let resolve = |id: &Ref| context.resolver.resolve_ref(id);

        if let Some((rel_name, rel_term)) = self.rel.value.split_once('-') {
            context.ns.has_relationship(
                context.dict,
                &Symbol::from(rel_name),
                &Some(Symbol::from(rel_term)),
                &self.ref_value,
                &resolve,
            )
        } else {
            context
                .ns
                .has_relationship(context.dict, &self.rel, &None, &self.ref_value, &resolve)
        }
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}?", self.rel.value)?;
        if let Some(ref_value) = &self.ref_value {
            write!(f, " {}", ref_value)
        } else {
            Ok(())
        }
    }
}

impl Visitable for Relation {
    fn accept_visitor(&self, visitor: &mut impl Visitor) {
        visitor.visit_relation(self);
    }
}
