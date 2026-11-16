// Copyright (c) 2026, J2 Innovations. All Rights Reserved

//! Haystack time span types, ported from Haxall's `Span` and `DateSpan`.
//!
//! - [`Span`]: a timezone-aware DateTime range (inclusive start, exclusive end), either absolute
//!   or relative (`today`, `thisWeek`, `pastMonth`, ...).
//! - [`DateSpan`]: a date-only range (inclusive start and end) with a period classification.
//!
//! Note the differing end semantics: a `Span` range excludes its end, so
//! `"2026-04-01,2026-05-01"` is the whole of April, while a `DateSpan` range includes its end,
//! so the same month is `"2026-04-01,2026-04-30"`.
//!
//! ```
//! use chrono_tz::Tz;
//! use libhaystack::span::*;
//!
//! // Parse an absolute date range in a timezone (the end date is exclusive)
//! let span = Span::from_str("2026-04-01,2026-05-01", Tz::Europe__London).expect("Span");
//! assert_eq!(span.num_days(), 30);
//! assert!(span.aligns_to_month());
//!
//! // Relative spans resolve against the current date
//! let this_week = Span::make_rel(SpanMode::ThisWeek, Tz::UTC);
//! assert!(this_week.mode().is_rel());
//! ```

pub mod date_span;
pub mod date_time_span;

pub use date_span::{DateSpan, DateSpanPeriod};
pub use date_time_span::{Span, SpanMode, SpanModePeriod};
