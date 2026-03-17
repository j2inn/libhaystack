// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack filter builder
//!
//! Provides a fluent, type-safe API for constructing [`Filter`](crate::filter::Filter) values
//! by building the filter's AST directly — no intermediate string parsing required.
//!
//! # Examples
//!
//! ```
//! use libhaystack::filter::{Filter, FilterBuilder};
//! use libhaystack::val::Value;
//!
//! // site and dis == "Main"
//! let filter = FilterBuilder::new()
//!     .has("site")
//!     .and()
//!     .eq("dis", Value::make_str("Main"))
//!     .build();
//!
//! assert_eq!(filter.to_string(), r#"site and dis == "Main""#);
//! ```
//!
//! ```
//! use libhaystack::filter::FilterBuilder;
//! use libhaystack::val::Value;
//!
//! // (equip or point) and siteRef == @mySite
//! let filter = FilterBuilder::new()
//!     .start_parens()
//!         .has("equip")
//!         .or()
//!         .has("point")
//!     .end_parens()
//!     .and()
//!     .eq("siteRef", Value::make_ref("mySite"))
//!     .build();
//!
//! assert_eq!(filter.to_string(), "(equip or point) and siteRef == @mySite");
//! ```

use super::nodes::{And, Cmp, CmpOp, Has, IsA, Missing, Or, Parens, Relation, Term, WildcardEq};
use super::path::Path;
use super::Filter;
use crate::haystack::encoding::zinc::decode::id::Id;
use crate::haystack::val::{Ref, Symbol, Value};
use std::marker::PhantomData;

/// Converts a value into a filter [`Path`].
///
/// Implemented for `&str` (single segment) and `&[&str]` / arrays (multi-segment).
pub trait IntoFilterPath {
    fn into_filter_path(self) -> Path;
}

impl IntoFilterPath for &str {
    fn into_filter_path(self) -> Path {
        Path::from(self)
    }
}

impl IntoFilterPath for &[&str] {
    fn into_filter_path(self) -> Path {
        Path::from(self.iter().map(|s| Id::from(*s)).collect::<Vec<_>>())
    }
}

impl<const N: usize> IntoFilterPath for [&str; N] {
    fn into_filter_path(self) -> Path {
        Path::from(self.iter().map(|s| Id::from(*s)).collect::<Vec<_>>())
    }
}

impl IntoFilterPath for Vec<&str> {
    fn into_filter_path(self) -> Path {
        Path::from(self.iter().map(|s| Id::from(*s)).collect::<Vec<_>>())
    }
}

/// Typestate marker for [`FilterBuilder`] — no term has been added yet.
///
/// In this state, connectives (`and`, `or`), `end_parens`, and `build` are not
/// available. Add a term first (e.g. [`has`](FilterBuilder::has),
/// [`eq`](FilterBuilder::eq), [`is_a`](FilterBuilder::is_a), …).
pub struct NeedsTerm;

/// Typestate marker for [`FilterBuilder`] — at least one term is pending.
///
/// In this state all methods are available, including connectives (`and`, `or`),
/// `end_parens`, and `build`.
pub struct HasTerm;

/// Fluent builder for constructing a Haystack [`Filter`] via its AST.
///
/// Uses a [typestate] pattern to prevent invalid filter sequences at compile
/// time. The builder starts in the [`NeedsTerm`] state; only term-producing
/// methods are available. After adding a term the builder transitions to
/// [`HasTerm`], where `and`, `or`, `end_parens`, and `build` become available.
///
/// # Invalid sequences that do not compile
///
/// ```compile_fail
/// # use libhaystack::filter::FilterBuilder;
/// // cannot call or() without a preceding term
/// let _ = FilterBuilder::new().or();
/// ```
///
/// ```compile_fail
/// # use libhaystack::filter::FilterBuilder;
/// // cannot call or() twice in a row
/// let _ = FilterBuilder::new().has("site").or().or();
/// ```
///
/// ```compile_fail
/// # use libhaystack::filter::FilterBuilder;
/// // cannot build an empty filter
/// let _ = FilterBuilder::new().build();
/// ```
///
/// # Path arguments
///
/// Any method that accepts a `path` parameter can receive:
/// - A single `&str` tag name: `"site"`
/// - A slice of `&str` for indirection: `["siteRef", "dis"]`
/// - A fixed-size array: `["equipRef", "siteRef"]`
///
/// # Value arguments
///
/// Comparison methods accept a [`Value`] directly. Use the `Value::make_*` helpers
/// (e.g. [`Value::make_str`], [`Value::make_number`], [`Value::make_ref`]) to construct
/// the right type.
///
/// [typestate]: https://cliffle.com/blog/rust-typestate/
pub struct FilterBuilder<S = NeedsTerm> {
    _state: PhantomData<S>,
    /// Completed `And` nodes for the top-level `Or`.
    ands: Vec<And>,
    /// Terms accumulating for the current `And` node.
    current_terms: Vec<Term>,
    /// Stack of saved `(ands, current_terms)` states pushed by `start_parens`.
    paren_stack: Vec<(Vec<And>, Vec<Term>)>,
}

impl FilterBuilder<NeedsTerm> {
    /// Create a new, empty [`FilterBuilder`] in the [`NeedsTerm`] state.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FilterBuilder<NeedsTerm> {
    fn default() -> Self {
        FilterBuilder {
            _state: PhantomData,
            ands: Vec::new(),
            current_terms: Vec::new(),
            paren_stack: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Methods available in any state (term producers + start_parens)
// ---------------------------------------------------------------------------

impl<S> FilterBuilder<S> {
    // -------------------------------------------------------------------------
    // Parentheses
    // -------------------------------------------------------------------------

    /// Opens a parenthesised sub-expression.
    ///
    /// The current builder state is saved onto an internal stack; subsequent
    /// method calls accumulate terms for the sub-expression until
    /// [`end_parens`](FilterBuilder::end_parens) is called.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// let f = FilterBuilder::new()
    ///     .start_parens()
    ///         .has("site")
    ///         .or()
    ///         .has("equip")
    ///     .end_parens()
    ///     .build();
    /// assert_eq!(f.to_string(), "(site or equip)");
    /// ```
    pub fn start_parens(mut self) -> FilterBuilder<NeedsTerm> {
        self.paren_stack.push((
            std::mem::take(&mut self.ands),
            std::mem::take(&mut self.current_terms),
        ));
        self.transition()
    }

    // -------------------------------------------------------------------------
    // Terminal terms
    // -------------------------------------------------------------------------

    /// Adds a *has* condition — the tag at `path` must exist.
    ///
    /// Equivalent to the bare path form in filter syntax: `site`.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// let f = FilterBuilder::new().has("site").build();
    /// assert_eq!(f.to_string(), "site");
    /// ```
    pub fn has(mut self, path: impl IntoFilterPath) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::Has(Has {
            path: path.into_filter_path(),
        }));
        self.transition()
    }

    /// Adds a *missing* condition — the tag at `path` must **not** exist.
    ///
    /// Equivalent to `not path` in filter syntax.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// let f = FilterBuilder::new().not("site").build();
    /// assert_eq!(f.to_string(), "not site");
    /// ```
    pub fn not(mut self, path: impl IntoFilterPath) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::Missing(Missing {
            path: path.into_filter_path(),
        }));
        self.transition()
    }

    /// Adds an *is-a* condition using a type symbol.
    ///
    /// Equivalent to `^symbol` in filter syntax.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// # use libhaystack::val::Symbol;
    /// let f = FilterBuilder::new().is_a("point").build();
    /// assert_eq!(f.to_string(), "^point");
    /// ```
    pub fn is_a(mut self, symbol: impl Into<Symbol>) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::IsA(IsA {
            symbol: symbol.into(),
        }));
        self.transition()
    }

    /// Adds a *wildcard equality* condition.
    ///
    /// Follows references at `id` until the target matches `ref_value`.
    /// Equivalent to `id *== @ref` in filter syntax.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// # use libhaystack::val::Ref;
    /// let f = FilterBuilder::new()
    ///     .wildcard_eq("siteRef", Ref::from("mySite"))
    ///     .build();
    /// assert_eq!(f.to_string(), "siteRef *== @mySite");
    /// ```
    pub fn wildcard_eq(
        mut self,
        id: impl IntoFilterPath,
        ref_value: Ref,
    ) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::WildcardEq(WildcardEq {
            id: id.into_filter_path(),
            ref_value,
        }));
        self.transition()
    }

    /// Adds a *relationship* condition.
    ///
    /// - `rel` – the relationship symbol name (e.g. `"containedBy"`)
    /// - `term` – optional relationship term symbol
    /// - `ref_value` – optional target reference
    ///
    /// Equivalent to `rel?`, `rel? ^term`, or `rel? ^term @ref` in filter syntax.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// # use libhaystack::val::{Symbol, Ref};
    /// let f = FilterBuilder::new()
    ///     .relation("containedBy", None, Some(Ref::from("mySite")))
    ///     .build();
    /// assert_eq!(f.to_string(), "containedBy? @mySite");
    /// ```
    pub fn relation(
        mut self,
        rel: impl Into<Symbol>,
        term: Option<Symbol>,
        ref_value: Option<Ref>,
    ) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::Relation(Relation {
            rel: rel.into(),
            rel_term: term,
            ref_value,
        }));
        self.transition()
    }

    // -------------------------------------------------------------------------
    // Comparison operators
    // -------------------------------------------------------------------------

    /// Adds an equality comparison: `path == value`.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// # use libhaystack::val::Value;
    /// let f = FilterBuilder::new().eq("dis", Value::make_str("Chiller")).build();
    /// assert_eq!(f.to_string(), r#"dis == "Chiller""#);
    /// ```
    pub fn eq(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::Eq, value)
    }

    /// Adds a not-equal comparison: `path != value`.
    pub fn ne(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::NotEq, value)
    }

    /// Adds a less-than comparison: `path < value`.
    pub fn lt(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::LessThan, value)
    }

    /// Adds a less-than-or-equal comparison: `path <= value`.
    pub fn lte(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::LessThanEq, value)
    }

    /// Adds a greater-than comparison: `path > value`.
    pub fn gt(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::GreatThan, value)
    }

    /// Adds a greater-than-or-equal comparison: `path >= value`.
    pub fn gte(self, path: impl IntoFilterPath, value: Value) -> FilterBuilder<HasTerm> {
        self.cmp(path, CmpOp::GreatThanEq, value)
    }

    // -------------------------------------------------------------------------
    // Sub-filter embedding
    // -------------------------------------------------------------------------

    /// Embeds an existing [`Filter`] as a parenthesised sub-expression.
    ///
    /// This is useful for composing pre-built filters together.
    ///
    /// ```
    /// # use libhaystack::filter::{Filter, FilterBuilder};
    /// # use libhaystack::val::Value;
    /// let inner = Filter::try_from("equip or point").unwrap();
    /// let f = FilterBuilder::new()
    ///     .filter(inner)
    ///     .and()
    ///     .has("site")
    ///     .build();
    /// assert_eq!(f.to_string(), "(equip or point) and site");
    /// ```
    pub fn filter(mut self, filter: Filter) -> FilterBuilder<HasTerm> {
        self.current_terms
            .push(Term::Parens(Parens { or: filter.or }));
        self.transition()
    }

    // -------------------------------------------------------------------------
    // Private helpers
    // -------------------------------------------------------------------------

    /// Reinterprets the builder's state marker without moving any data.
    fn transition<T>(self) -> FilterBuilder<T> {
        FilterBuilder {
            _state: PhantomData,
            ands: self.ands,
            current_terms: self.current_terms,
            paren_stack: self.paren_stack,
        }
    }

    /// Finalises the accumulated terms into an `And` node and clears `current_terms`.
    /// Does nothing if `current_terms` is empty.
    fn flush_and(&mut self) {
        if !self.current_terms.is_empty() {
            self.ands.push(And {
                terms: std::mem::take(&mut self.current_terms),
            });
        }
    }

    fn cmp(mut self, path: impl IntoFilterPath, op: CmpOp, value: Value) -> FilterBuilder<HasTerm> {
        self.current_terms.push(Term::Cmp(Cmp {
            path: path.into_filter_path(),
            op,
            value,
        }));
        self.transition()
    }
}

// ---------------------------------------------------------------------------
// Methods available only when a term has been added
// ---------------------------------------------------------------------------

impl FilterBuilder<HasTerm> {
    // -------------------------------------------------------------------------
    // Logical connectives
    // -------------------------------------------------------------------------

    /// Separator between conditions in the same `and` group.
    ///
    /// In the AST, consecutive terms are implicitly and-ed, so this is a no-op
    /// that exists purely for readability.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// let f = FilterBuilder::new().has("site").and().has("equip").build();
    /// assert_eq!(f.to_string(), "site and equip");
    /// ```
    pub fn and(self) -> FilterBuilder<HasTerm> {
        self
    }

    /// Finalises the current `and`-group and starts a new one, producing an `or` in the output.
    ///
    /// ```
    /// # use libhaystack::filter::FilterBuilder;
    /// let f = FilterBuilder::new().has("site").or().has("equip").build();
    /// assert_eq!(f.to_string(), "site or equip");
    /// ```
    pub fn or(mut self) -> FilterBuilder<NeedsTerm> {
        self.flush_and();
        self.transition()
    }

    // -------------------------------------------------------------------------
    // Parentheses
    // -------------------------------------------------------------------------

    /// Closes the current parenthesised sub-expression and adds it as a single
    /// [`Parens`] term in the outer context.
    ///
    /// If called without a matching [`start_parens`](FilterBuilder::start_parens)
    /// the call is silently ignored and the builder state is left unchanged.
    pub fn end_parens(mut self) -> FilterBuilder<HasTerm> {
        if let Some((outer_ands, outer_terms)) = self.paren_stack.pop() {
            self.flush_and();
            let inner_or = Or {
                ands: std::mem::take(&mut self.ands),
            };
            self.ands = outer_ands;
            self.current_terms = outer_terms;
            self.current_terms
                .push(Term::Parens(Parens { or: inner_or }));
        }
        self
    }

    // -------------------------------------------------------------------------
    // Build
    // -------------------------------------------------------------------------

    /// Consumes the builder and returns the constructed [`Filter`].
    ///
    /// Any open `and`-group is automatically finalised.
    pub fn build(mut self) -> Filter {
        self.flush_and();
        Filter {
            or: Or { ands: self.ands },
        }
    }
}

/// Convert a [`FilterBuilder<HasTerm>`] into a [`Filter`].
///
/// Enforces at compile time that only a builder with at least one term can be
/// converted — an empty builder (in [`NeedsTerm`] state) does not implement this.
impl From<FilterBuilder<HasTerm>> for Filter {
    fn from(builder: FilterBuilder<HasTerm>) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::val::{Date, Ref, Time, Value};

    // ------------------------------------------------------------------
    // has / not
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_has() {
        let f = FilterBuilder::new().has("site").build();
        assert_eq!(f.to_string(), "site");
    }

    #[test]
    fn test_builder_not() {
        let f = FilterBuilder::new().not("site").build();
        assert_eq!(f.to_string(), "not site");
    }

    // ------------------------------------------------------------------
    // and / or
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_and() {
        let f = FilterBuilder::new().has("site").and().has("equip").build();
        assert_eq!(f.to_string(), "site and equip");
    }

    #[test]
    fn test_builder_or() {
        let f = FilterBuilder::new().has("site").or().has("equip").build();
        assert_eq!(f.to_string(), "site or equip");
    }

    #[test]
    fn test_builder_and_or_combined() {
        // site and equip or point
        let f = FilterBuilder::new()
            .has("site")
            .and()
            .has("equip")
            .or()
            .has("point")
            .build();
        assert_eq!(f.to_string(), "site and equip or point");
    }

    // ------------------------------------------------------------------
    // is_a
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_is_a() {
        let f = FilterBuilder::new().is_a("point").build();
        assert_eq!(f.to_string(), "^point");
    }

    // ------------------------------------------------------------------
    // Parentheses
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_parens() {
        let f = FilterBuilder::new()
            .start_parens()
            .has("equip")
            .or()
            .has("point")
            .end_parens()
            .build();
        assert_eq!(f.to_string(), "(equip or point)");
    }

    #[test]
    fn test_builder_parens_with_outer_term() {
        let f = FilterBuilder::new()
            .start_parens()
            .has("equip")
            .or()
            .has("point")
            .end_parens()
            .and()
            .eq("siteRef", Value::make_ref("mySite"))
            .build();
        assert_eq!(f.to_string(), "(equip or point) and siteRef == @mySite");
    }

    // ------------------------------------------------------------------
    // Comparison operators
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_eq_str() {
        let f = FilterBuilder::new()
            .eq("dis", Value::make_str("Chiller"))
            .build();
        assert_eq!(f.to_string(), r#"dis == "Chiller""#);
    }

    #[test]
    fn test_builder_eq_number() {
        let f = FilterBuilder::new()
            .eq("num", Value::make_number(42.0))
            .build();
        assert_eq!(f.to_string(), "num == 42");
    }

    #[test]
    fn test_builder_ne() {
        let f = FilterBuilder::new()
            .ne("num", Value::make_number(42.0))
            .build();
        assert_eq!(f.to_string(), "num != 42");
    }

    #[test]
    fn test_builder_lt() {
        let f = FilterBuilder::new()
            .lt("num", Value::make_number(10.0))
            .build();
        assert_eq!(f.to_string(), "num < 10");
    }

    #[test]
    fn test_builder_lte() {
        let f = FilterBuilder::new()
            .lte("num", Value::make_number(10.0))
            .build();
        assert_eq!(f.to_string(), "num <= 10");
    }

    #[test]
    fn test_builder_gt() {
        let f = FilterBuilder::new()
            .gt("num", Value::make_number(10.0))
            .build();
        assert_eq!(f.to_string(), "num > 10");
    }

    #[test]
    fn test_builder_gte() {
        let f = FilterBuilder::new()
            .gte("num", Value::make_number(10.0))
            .build();
        assert_eq!(f.to_string(), "num >= 10");
    }

    #[test]
    fn test_builder_eq_ref() {
        let f = FilterBuilder::new()
            .eq("siteRef", Value::make_ref("mySite"))
            .build();
        assert_eq!(f.to_string(), "siteRef == @mySite");
    }

    #[test]
    fn test_builder_eq_bool() {
        let f = FilterBuilder::new()
            .eq("active", Value::make_bool(true))
            .build();
        assert_eq!(f.to_string(), "active == true");
    }

    #[test]
    fn test_builder_eq_date() {
        let date = Date::from_ymd(2024, 3, 15).expect("date");
        let f = FilterBuilder::new()
            .eq("lastMod", Value::make_date(date))
            .build();
        assert_eq!(f.to_string(), "lastMod == 2024-03-15");
    }

    #[test]
    fn test_builder_eq_time() {
        let time = Time::from_hms(12, 30, 0).expect("time");
        let f = FilterBuilder::new()
            .eq("startTime", Value::make_time(time))
            .build();
        assert_eq!(f.to_string(), "startTime == 12:30:00");
    }

    // ------------------------------------------------------------------
    // Multi-segment paths
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_multi_segment_path_has() {
        let f = FilterBuilder::new().has(["siteRef", "dis"]).build();
        assert_eq!(f.to_string(), "siteRef->dis");
    }

    #[test]
    fn test_builder_multi_segment_path_eq() {
        let f = FilterBuilder::new()
            .eq(["siteRef", "dis"], Value::make_str("Main"))
            .build();
        assert_eq!(f.to_string(), r#"siteRef->dis == "Main""#);
    }

    // ------------------------------------------------------------------
    // wildcard_eq
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_wildcard_eq() {
        let f = FilterBuilder::new()
            .wildcard_eq("siteRef", Ref::from("mySite"))
            .build();
        assert_eq!(f.to_string(), "siteRef *== @mySite");
    }

    // ------------------------------------------------------------------
    // relation
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_relation_simple() {
        let f = FilterBuilder::new()
            .relation("containedBy", None, None)
            .build();
        assert_eq!(f.to_string(), "containedBy?");
    }

    #[test]
    fn test_builder_relation_with_ref() {
        let f = FilterBuilder::new()
            .relation("containedBy", None, Some(Ref::from("mySite")))
            .build();
        assert_eq!(f.to_string(), "containedBy? @mySite");
    }

    #[test]
    fn test_builder_relation_with_term_and_ref() {
        let f = FilterBuilder::new()
            .relation(
                "containedBy",
                Some(Symbol::from("site")),
                Some(Ref::from("mySite")),
            )
            .build();
        assert_eq!(f.to_string(), "containedBy? ^site @mySite");
    }

    // ------------------------------------------------------------------
    // Embedded filter
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_embed_filter() {
        let inner = Filter::try_from("equip or point").unwrap();
        let f = FilterBuilder::new().filter(inner).and().has("site").build();
        assert_eq!(f.to_string(), "(equip or point) and site");
    }

    // ------------------------------------------------------------------
    // From<FilterBuilder> for Filter
    // ------------------------------------------------------------------

    #[test]
    fn test_filter_from_builder() {
        let builder: FilterBuilder<HasTerm> = FilterBuilder::new().has("site").and().has("equip");
        let f: Filter = builder.into();
        assert_eq!(f.to_string(), "site and equip");
    }

    // ------------------------------------------------------------------
    // Round-trip: builder output matches parser output
    // ------------------------------------------------------------------

    #[test]
    fn test_builder_round_trip_simple() {
        let built = FilterBuilder::new()
            .has("site")
            .and()
            .eq("dis", Value::make_str("Test"))
            .build();
        let parsed = Filter::try_from(r#"site and dis == "Test""#).unwrap();
        assert_eq!(built.to_string(), parsed.to_string());
    }

    #[test]
    fn test_builder_round_trip_parens() {
        let built = FilterBuilder::new()
            .start_parens()
            .has("equip")
            .or()
            .has("point")
            .end_parens()
            .and()
            .has("site")
            .build();
        let parsed = Filter::try_from("(equip or point) and site").unwrap();
        assert_eq!(built.to_string(), parsed.to_string());
    }

    #[test]
    fn test_builder_round_trip_or() {
        let built = FilterBuilder::new()
            .has("site")
            .or()
            .has("equip")
            .or()
            .has("point")
            .build();
        let parsed = Filter::try_from("site or equip or point").unwrap();
        assert_eq!(built.to_string(), parsed.to_string());
    }
}
