// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter node types

use super::eval::{Eval, EvalContext};
use super::path::Path;
use super::resolver::PathResolver;
use crate::haystack::val::{Ref, Value};
use crate::val::Symbol;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

/// A Haystack Filter Or expression
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Or {
    pub(super) ands: Vec<And>,
}

impl Display for Or {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.ands.iter().enumerate().try_for_each(|it| -> Result {
            it.1.fmt(f)?;
            if it.0 < self.ands.len() - 1 {
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

/// A Haystack Filter And expression
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct And {
    pub(super) terms: Vec<Term>,
}

impl Eval for And {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        self.terms.iter().all(|term| term.eval(context))
    }
}

impl Display for And {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.terms.iter().enumerate().try_for_each(|it| -> Result {
            it.1.fmt(f)?;
            if it.0 < self.terms.len() - 1 {
                f.write_str(" and ")
            } else {
                Ok(())
            }
        })
    }
}

/// A Haystack Filter terminal
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) enum Term {
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

/// Filter compare operators
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) enum CmpOp {
    Eq,
    NotEq,
    LessThan,
    LessThanEq,
    GreatThan,
    GreatThanEq,
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Cmp {
    pub(super) path: Path,
    pub(super) op: CmpOp,
    pub(super) value: Value,
}
impl Eval for Cmp {
    fn eval<R: PathResolver>(&self, context: &EvalContext<R>) -> bool {
        match self.op {
            CmpOp::Eq => context.resolve(&self.path) == self.value,
            CmpOp::NotEq => context.resolve(&self.path) != self.value,
            CmpOp::LessThan => context.resolve(&self.path) < self.value,
            CmpOp::LessThanEq => context.resolve(&self.path) <= self.value,
            CmpOp::GreatThan => context.resolve(&self.path) > self.value,
            CmpOp::GreatThanEq => context.resolve(&self.path) >= self.value,
        }
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

/// Missing term
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Missing {
    pub(super) path: Path,
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

/// Parenthesis term
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Parens {
    pub(super) or: Or,
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

/// Has term
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Has {
    pub(super) path: Path,
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

/// Is A term
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct IsA {
    pub(super) symbol: Symbol,
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

/// Wildcard equality
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct WildcardEq {
    pub(super) id: Path,
    pub(super) ref_value: Ref,
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

/// Relation
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub(super) struct Relation {
    pub(super) rel: Symbol,
    pub(super) ref_value: Option<Ref>,
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
