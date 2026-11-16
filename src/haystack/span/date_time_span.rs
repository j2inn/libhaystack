// Copyright (c) 2026, Siemens. All Rights Reserved

//! Timezone-aware DateTime span type (inclusive start, exclusive end).
//!
//! A `Span` represents a DateTime range with
//! timezone awareness. Unlike `DateSpan` (see [`DateSpan`](super::DateSpan)), `Span` operates at
//! DateTime granularity with an **exclusive end** (start is inclusive, end is exclusive).
//!
//! `Span` can be either absolute (fixed start/end DateTime values) or relative (a mode like
//! `today`, `thisWeek`, `pastMonth` that resolves to absolute dates when `fresh()` is called).
//!
//! # Timezones
//!
//! A `Span` does not store a timezone of its own: it is carried by `start` and `end`, which are
//! required to share the same IANA timezone (enforced by [`Span::make_abs`]). All date-level
//! operations — [`Span::to_date_span`], [`Span::num_days`], [`Span::dis`], [`Span::prev`],
//! [`Span::next`] and the alignment predicates — are evaluated against the *local* calendar dates
//! in that timezone, so they stay correct across DST transitions (a 23- or 25-hour day still
//! counts as one day).

use crate::val::{Date, DateTime};
use chrono::{
    Datelike, Days, FixedOffset, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta,
    TimeZone, Weekday,
};
use chrono_tz::Tz;

use super::date_span::{DateSpan, DateSpanPeriod, fmt_date_dd_mmm_yyyy};

/// The weekday a week-aligned span starts on. Fantom uses the current locale's start of week;
/// this port fixes it to Sunday, matching [`DateSpan`]'s own week alignment.
const START_OF_WEEK: Weekday = Weekday::Sun;

/// Formats a `DateTime` as `"DD-MMM-YYYY HH:MM"`, e.g. `"15-Jul-2026 14:30"` — used by
/// `Span::dis()` for a span that isn't aligned to midnight date boundaries.
fn fmt_datetime_dd_mmm_yyyy_hm(dt: &DateTime) -> String {
    format!(
        "{} {:02}:{:02}",
        fmt_date_dd_mmm_yyyy(&Date::from(dt.date_naive())),
        dt.hour(),
        dt.minute()
    )
}

/// Adds `days` calendar days to a date, saturating at the representable range.
///
/// Calendar arithmetic, unlike adding `24h` to a DateTime, is unaffected by DST transitions.
fn add_days(date: NaiveDate, days: u64) -> NaiveDate {
    date.checked_add_days(Days::new(days))
        .unwrap_or(NaiveDate::MAX)
}

/// Subtracts `days` calendar days from a date, saturating at the representable range.
fn sub_days(date: NaiveDate, days: u64) -> NaiveDate {
    date.checked_sub_days(Days::new(days))
        .unwrap_or(NaiveDate::MIN)
}

/// The first day of `date`'s month.
fn first_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

/// The first day of the month following `date`'s month.
fn first_of_next_month(date: NaiveDate) -> NaiveDate {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1)
    } else {
        (date.year(), date.month() + 1)
    };
    NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(date)
}

/// The first day of the quarter containing `date`.
fn first_of_quarter(date: NaiveDate) -> NaiveDate {
    let month = (date.month() - 1) / 3 * 3 + 1;
    NaiveDate::from_ymd_opt(date.year(), month, 1).unwrap_or(date)
}

/// The first day of the quarter following the one containing `date`.
fn first_of_next_quarter(date: NaiveDate) -> NaiveDate {
    let start = first_of_quarter(date);
    let month = start.month() + 3;
    if month > 12 {
        NaiveDate::from_ymd_opt(start.year() + 1, 1, 1).unwrap_or(date)
    } else {
        NaiveDate::from_ymd_opt(start.year(), month, 1).unwrap_or(date)
    }
}

/// January 1st of the given year.
fn first_of_year(year: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or(NaiveDate::MIN)
}

/// The mode of a [`Span`] — either absolute or one of the relative period shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanMode {
    /// Absolute span with fixed start/end.
    Abs,
    /// Today (midnight to midnight).
    Today,
    /// Yesterday.
    Yesterday,
    /// This week (start of week to the same weekday next week).
    ThisWeek,
    /// Last week (previous week, same alignment as `ThisWeek`).
    LastWeek,
    /// Past 7 days, ending at today's midnight (exclusive, so today is not included).
    PastWeek,
    /// This month (1st to 1st of next month).
    ThisMonth,
    /// Last month.
    LastMonth,
    /// Past 28 days, ending at today's midnight (exclusive).
    PastMonth,
    /// This quarter.
    ThisQuarter,
    /// Last quarter.
    LastQuarter,
    /// Past 90 days, ending at today's midnight (exclusive).
    PastQuarter,
    /// This year (Jan 1 to next Jan 1).
    ThisYear,
    /// Last year.
    LastYear,
    /// Past 365 days, ending at today's midnight (exclusive).
    PastYear,
}

impl SpanMode {
    /// Whether this mode is relative (needs `fresh()` to resolve to absolute dates).
    pub fn is_rel(&self) -> bool {
        !matches!(self, SpanMode::Abs)
    }

    /// Whether this mode is absolute.
    pub fn is_abs(&self) -> bool {
        matches!(self, SpanMode::Abs)
    }

    /// The period this mode represents.
    pub fn period(&self) -> SpanModePeriod {
        match self {
            SpanMode::Abs => SpanModePeriod::Abs,
            SpanMode::Today | SpanMode::Yesterday => SpanModePeriod::Day,
            SpanMode::ThisWeek | SpanMode::LastWeek | SpanMode::PastWeek => SpanModePeriod::Week,
            SpanMode::ThisMonth | SpanMode::LastMonth | SpanMode::PastMonth => {
                SpanModePeriod::Month
            }
            SpanMode::ThisQuarter | SpanMode::LastQuarter | SpanMode::PastQuarter => {
                SpanModePeriod::Quarter
            }
            SpanMode::ThisYear | SpanMode::LastYear | SpanMode::PastYear => SpanModePeriod::Year,
        }
    }

    /// Display name for the mode.
    pub fn dis(&self) -> &'static str {
        match self {
            SpanMode::Abs => "abs",
            SpanMode::Today => "today",
            SpanMode::Yesterday => "yesterday",
            SpanMode::ThisWeek => "thisWeek",
            SpanMode::LastWeek => "lastWeek",
            SpanMode::PastWeek => "pastWeek",
            SpanMode::ThisMonth => "thisMonth",
            SpanMode::LastMonth => "lastMonth",
            SpanMode::PastMonth => "pastMonth",
            SpanMode::ThisQuarter => "thisQuarter",
            SpanMode::LastQuarter => "lastQuarter",
            SpanMode::PastQuarter => "pastQuarter",
            SpanMode::ThisYear => "thisYear",
            SpanMode::LastYear => "lastYear",
            SpanMode::PastYear => "pastYear",
        }
    }

    /// Parse a mode from its display name.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "abs" => Some(SpanMode::Abs),
            "today" => Some(SpanMode::Today),
            "yesterday" => Some(SpanMode::Yesterday),
            "thisWeek" => Some(SpanMode::ThisWeek),
            "lastWeek" => Some(SpanMode::LastWeek),
            "pastWeek" => Some(SpanMode::PastWeek),
            "thisMonth" => Some(SpanMode::ThisMonth),
            "lastMonth" => Some(SpanMode::LastMonth),
            "pastMonth" => Some(SpanMode::PastMonth),
            "thisQuarter" => Some(SpanMode::ThisQuarter),
            "lastQuarter" => Some(SpanMode::LastQuarter),
            "pastQuarter" => Some(SpanMode::PastQuarter),
            "thisYear" => Some(SpanMode::ThisYear),
            "lastYear" => Some(SpanMode::LastYear),
            "pastYear" => Some(SpanMode::PastYear),
            _ => None,
        }
    }
}

/// The period granularity of a [`SpanMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanModePeriod {
    Abs,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

/// A timezone-aware DateTime span.
///
/// Key invariants, enforced by the constructors:
/// - `start` is **inclusive**
/// - `end` is **exclusive** (this is the main difference from `DateSpan`, which is inclusive on
///   both ends)
/// - `start <= end`
/// - Both `start` and `end` are in the same timezone
///
/// A `Span` can be absolute (fixed dates) or relative (a `SpanMode` that resolves lazily).
/// Call `fresh()` to resolve a relative span to absolute dates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    start: DateTime,
    end: DateTime,
    mode: SpanMode,
    aligns_to_dates: bool,
}

impl Span {
    /// Create an absolute span from two DateTime values.
    ///
    /// Returns an error if `start` and `end` are in different timezones, or if `start > end`.
    pub fn make_abs(start: DateTime, end: DateTime) -> Result<Self, String> {
        if start.timezone() != end.timezone() {
            return Err(format!(
                "Mismatched tz: {} != {}",
                start.timezone(),
                end.timezone()
            ));
        }
        if start > end {
            return Err("start > end".into());
        }
        Ok(Span::make_date_times(SpanMode::Abs, start, end))
    }

    /// Internal constructor from two DateTime values known to satisfy the invariants.
    fn make_date_times(mode: SpanMode, start: DateTime, end: DateTime) -> Self {
        let aligns_to_dates = is_midnight(&start) && is_midnight(&end);
        Span {
            start,
            end,
            mode,
            aligns_to_dates,
        }
    }

    /// Internal constructor from an inclusive start date and an **exclusive** end date, both
    /// resolved to midnight in `tz`.
    fn make_dates(mode: SpanMode, start: NaiveDate, end: NaiveDate, tz: Tz) -> Self {
        Span {
            start: date_at_midnight(start, tz),
            end: date_at_midnight(end, tz),
            mode,
            aligns_to_dates: true,
        }
    }

    /// Create a span for a single date in the given timezone (midnight to midnight).
    pub fn make_date(date: Date, tz: Tz) -> Self {
        Span::make_dates(SpanMode::Abs, *date, add_days(*date, 1), tz)
    }

    /// Create a relative span for the given mode and timezone, resolved against the current
    /// date in that timezone.
    pub fn make_rel(mode: SpanMode, tz: Tz) -> Self {
        let today = chrono::Utc::now().with_timezone(&tz).date_naive();
        Span::do_make_rel(mode, today, tz)
    }

    /// Resolve a relative mode against a given "today" date in `tz`.
    fn do_make_rel(mode: SpanMode, today: NaiveDate, tz: Tz) -> Self {
        let (start, end) = match mode {
            // An absolute mode has no period to resolve; treat it as a single day.
            SpanMode::Abs => (today, add_days(today, 1)),
            SpanMode::Today => (today, add_days(today, 1)),
            SpanMode::Yesterday => (sub_days(today, 1), today),
            SpanMode::ThisWeek => {
                let start = start_of_week(today);
                (start, add_days(start, 7))
            }
            SpanMode::LastWeek => {
                let start = sub_days(start_of_week(today), 7);
                (start, add_days(start, 7))
            }
            SpanMode::PastWeek => (sub_days(today, 7), today),
            SpanMode::ThisMonth => (first_of_month(today), first_of_next_month(today)),
            SpanMode::LastMonth => {
                let last = sub_days(first_of_month(today), 1);
                (first_of_month(last), first_of_next_month(last))
            }
            SpanMode::PastMonth => (sub_days(today, 28), today),
            SpanMode::ThisQuarter => (first_of_quarter(today), first_of_next_quarter(today)),
            SpanMode::LastQuarter => {
                let last = sub_days(first_of_quarter(today), 1);
                (first_of_quarter(last), first_of_next_quarter(last))
            }
            SpanMode::PastQuarter => (sub_days(today, 90), today),
            SpanMode::ThisYear => (first_of_year(today.year()), first_of_year(today.year() + 1)),
            SpanMode::LastYear => (first_of_year(today.year() - 1), first_of_year(today.year())),
            SpanMode::PastYear => (sub_days(today, 365), today),
        };
        Span::make_dates(mode, start, end, tz)
    }

    /// Create a "today" span for the given timezone.
    pub fn today(tz: Tz) -> Self {
        Span::make_rel(SpanMode::Today, tz)
    }

    /// Resolve a relative span to its current absolute dates.
    /// For absolute spans, this is a no-op (returns a clone).
    pub fn fresh(&self) -> Self {
        if self.mode.is_abs() {
            return self.clone();
        }
        Span::make_rel(self.mode, self.tz())
    }

    /// Inclusive start DateTime.
    pub fn start(&self) -> &DateTime {
        &self.start
    }

    /// Exclusive end DateTime.
    pub fn end(&self) -> &DateTime {
        &self.end
    }

    /// Whether this span is absolute or relative.
    pub fn mode(&self) -> SpanMode {
        self.mode
    }

    /// Get the timezone of this span. Both `start` and `end` are in this timezone.
    pub fn tz(&self) -> Tz {
        self.start.timezone()
    }

    /// The local start date in this span's timezone.
    pub fn start_date(&self) -> Date {
        Date::from(self.start.date_naive())
    }

    /// The local **exclusive** end date in this span's timezone.
    pub fn end_date(&self) -> Date {
        Date::from(self.end.date_naive())
    }

    /// Convert to a different timezone, keeping the same instants.
    ///
    /// The result is always absolute, and its alignment is recomputed — a span aligned to
    /// midnight in one timezone is generally not aligned to midnight in another.
    pub fn to_time_zone(&self, tz: Tz) -> Self {
        if self.tz() == tz {
            return self.clone();
        }
        Span::make_date_times(
            SpanMode::Abs,
            DateTime::from(self.start.with_iana_timezone(tz)),
            DateTime::from(self.end.with_iana_timezone(tz)),
        )
    }

    /// Convert to a `DateSpan` using the aligned local dates.
    pub fn to_date_span(&self) -> DateSpan {
        let start = self.start.date_naive();
        if !self.aligns_to_dates {
            // Not midnight-aligned: both local dates are taken as-is (inclusive).
            return DateSpan::range(Date::from(start), self.end_date());
        }
        if self.aligns_to_day() {
            return DateSpan::from_period(Date::from(start), DateSpanPeriod::Day);
        }
        if self.aligns_to_week() {
            return DateSpan::from_period(Date::from(start), DateSpanPeriod::Week);
        }
        if self.aligns_to_month() {
            return DateSpan::from_period(Date::from(start), DateSpanPeriod::Month);
        }
        if self.aligns_to_year() {
            return DateSpan::from_period(Date::from(start), DateSpanPeriod::Year);
        }
        // The exclusive end date becomes an inclusive one, using calendar arithmetic.
        DateSpan::range(
            Date::from(start),
            Date::from(sub_days(self.end.date_naive(), 1)),
        )
    }

    /// Check if a DateTime is contained within this span (inclusive start, exclusive end).
    pub fn contains(&self, ts: &DateTime) -> bool {
        *ts >= self.start && *ts < self.end
    }

    /// Number of whole days this span includes, counted as *calendar* days in this span's
    /// timezone (so a DST transition does not add or drop a day).
    pub fn num_days(&self) -> i64 {
        (self.end.date_naive() - self.start.date_naive()).num_days()
    }

    /// Which quarter (1-4) this span falls in (based on the local start date).
    pub fn quarter(&self) -> i32 {
        ((self.start.date_naive().month() - 1) / 3 + 1) as i32
    }

    /// Whether this span aligns to midnight date boundaries in its timezone.
    pub fn aligns_to_dates(&self) -> bool {
        self.aligns_to_dates
    }

    /// Does this span align to a single day?
    pub fn aligns_to_day(&self) -> bool {
        self.aligns_to_dates && self.num_days() <= 1
    }

    /// Does this span align to a single week starting on [`START_OF_WEEK`]?
    pub fn aligns_to_week(&self) -> bool {
        self.aligns_to_dates
            && self.start.date_naive().weekday() == START_OF_WEEK
            && self.num_days() == 7
    }

    /// Does this span align to a single month (1st to the 1st of the next month)?
    pub fn aligns_to_month(&self) -> bool {
        if !self.aligns_to_dates {
            return false;
        }
        let (start, end) = (self.start.date_naive(), self.end.date_naive());
        if start.day() != 1 || end.day() != 1 {
            return false;
        }
        first_of_next_month(start) == end
    }

    /// Does this span align to a single quarter (1st of the quarter to the 1st of the next)?
    pub fn aligns_to_quarter(&self) -> bool {
        if !self.aligns_to_dates {
            return false;
        }
        let (start, end) = (self.start.date_naive(), self.end.date_naive());
        if start.day() != 1 || end.day() != 1 {
            return false;
        }
        if (start.month() - 1) % 3 != 0 {
            return false;
        }
        first_of_next_quarter(start) == end
    }

    /// Does this span align to a single year (Jan 1 to the next Jan 1)?
    pub fn aligns_to_year(&self) -> bool {
        if !self.aligns_to_dates {
            return false;
        }
        let (start, end) = (self.start.date_naive(), self.end.date_naive());
        start.day() == 1
            && end.day() == 1
            && start.month() == 1
            && end.month() == 1
            && start.year() + 1 == end.year()
    }

    /// Get the previous period, relative to this span's current alignment.
    ///
    /// For the `this*` relative modes this returns the matching `last*` relative span.
    pub fn prev(&self) -> Self {
        let tz = self.tz();
        match self.mode {
            SpanMode::Today => return Span::make_rel(SpanMode::Yesterday, tz),
            SpanMode::ThisWeek => return Span::make_rel(SpanMode::LastWeek, tz),
            SpanMode::ThisMonth => return Span::make_rel(SpanMode::LastMonth, tz),
            SpanMode::ThisQuarter => return Span::make_rel(SpanMode::LastQuarter, tz),
            SpanMode::ThisYear => return Span::make_rel(SpanMode::LastYear, tz),
            _ => {}
        }

        let (start, end) = (self.start.date_naive(), self.end.date_naive());
        if self.aligns_to_day() {
            return Span::make_dates(SpanMode::Abs, sub_days(start, 1), sub_days(end, 1), tz);
        }
        if self.aligns_to_week() {
            return Span::make_dates(SpanMode::Abs, sub_days(start, 7), sub_days(end, 7), tz);
        }
        if self.aligns_to_month() {
            let prev_start = first_of_month(sub_days(start, 1));
            return Span::make_dates(SpanMode::Abs, prev_start, start, tz);
        }
        if self.aligns_to_quarter() {
            let prev_start = first_of_quarter(sub_days(start, 1));
            return Span::make_dates(SpanMode::Abs, prev_start, start, tz);
        }
        if self.aligns_to_year() {
            return Span::make_dates(
                SpanMode::Abs,
                first_of_year(start.year() - 1),
                first_of_year(start.year()),
                tz,
            );
        }
        if self.aligns_to_dates {
            let days = self.num_days().unsigned_abs();
            return Span::make_dates(
                SpanMode::Abs,
                sub_days(start, days),
                sub_days(end, days),
                tz,
            );
        }
        let diff = *self.end - *self.start;
        Span::make_date_times(
            SpanMode::Abs,
            DateTime::from(*self.start - diff),
            DateTime::from(*self.end - diff),
        )
    }

    /// Get the next period, relative to this span's current alignment.
    ///
    /// For the `last*` relative modes this returns the matching `this*` relative span.
    pub fn next(&self) -> Self {
        let tz = self.tz();
        match self.mode {
            SpanMode::Yesterday => return Span::make_rel(SpanMode::Today, tz),
            SpanMode::LastWeek => return Span::make_rel(SpanMode::ThisWeek, tz),
            SpanMode::LastMonth => return Span::make_rel(SpanMode::ThisMonth, tz),
            SpanMode::LastQuarter => return Span::make_rel(SpanMode::ThisQuarter, tz),
            SpanMode::LastYear => return Span::make_rel(SpanMode::ThisYear, tz),
            _ => {}
        }

        let (start, end) = (self.start.date_naive(), self.end.date_naive());
        if self.aligns_to_day() {
            return Span::make_dates(SpanMode::Abs, add_days(start, 1), add_days(end, 1), tz);
        }
        if self.aligns_to_week() {
            return Span::make_dates(SpanMode::Abs, add_days(start, 7), add_days(end, 7), tz);
        }
        if self.aligns_to_month() {
            return Span::make_dates(SpanMode::Abs, end, first_of_next_month(end), tz);
        }
        if self.aligns_to_quarter() {
            return Span::make_dates(SpanMode::Abs, end, first_of_next_quarter(end), tz);
        }
        if self.aligns_to_year() {
            return Span::make_dates(
                SpanMode::Abs,
                first_of_year(end.year()),
                first_of_year(end.year() + 1),
                tz,
            );
        }
        if self.aligns_to_dates {
            let days = self.num_days().unsigned_abs();
            return Span::make_dates(
                SpanMode::Abs,
                add_days(start, days),
                add_days(end, days),
                tz,
            );
        }
        let diff = *self.end - *self.start;
        Span::make_date_times(
            SpanMode::Abs,
            DateTime::from(*self.start + diff),
            DateTime::from(*self.end + diff),
        )
    }

    /// Iterate over each day in the span (delegates to `to_date_span().each_day`).
    pub fn each_day<F>(&self, func: F)
    where
        F: FnMut(Date, i32),
    {
        self.to_date_span().each_day(func);
    }

    /// Machine-parseable string representation, round-trippable through
    /// [`from_str`](Span::from_str) for the same timezone.
    ///
    /// - relative span: the mode name, e.g. `"thisWeek"`
    /// - single day: `"YYYY-MM-DD"`
    /// - midnight-aligned range: `"YYYY-MM-DD,YYYY-MM-DD"` (the end date is **exclusive**)
    /// - anything else: `"<rfc3339>,<rfc3339>"`
    pub fn to_str(&self) -> String {
        if self.mode.is_rel() {
            return self.mode.dis().to_string();
        }
        if self.aligns_to_dates {
            if self.aligns_to_day() {
                return self.start.date_naive().format("%Y-%m-%d").to_string();
            }
            return format!(
                "{},{}",
                self.start.date_naive().format("%Y-%m-%d"),
                self.end.date_naive().format("%Y-%m-%d")
            );
        }
        format!("{},{}", self.start.to_rfc3339(), self.end.to_rfc3339())
    }

    /// Human-friendly display string for the console/REPL, e.g. `"01-Jul-2010..03-Jul-2010"`.
    /// Always renders the explicit inclusive `start..end` date range (or datetime range, for a
    /// span not aligned to midnight boundaries) with a zero-padded day and 4-digit year,
    /// matching the format used by the Funcs.xeto docs' own examples, regardless of mode.
    pub fn dis(&self) -> String {
        if !self.aligns_to_dates {
            return format!(
                "{}..{}",
                fmt_datetime_dd_mmm_yyyy_hm(&self.start),
                fmt_datetime_dd_mmm_yyyy_hm(&self.end)
            );
        }
        let date_span = self.to_date_span();
        format!(
            "{}..{}",
            fmt_date_dd_mmm_yyyy(&date_span.start),
            fmt_date_dd_mmm_yyyy(&date_span.end)
        )
    }

    /// Parse a `Span` from a string, resolving dates and local datetimes in `tz`.
    ///
    /// Supported formats:
    /// - mode keywords: `"today"`, `"thisWeek"`, `"pastMonth"`, etc.
    /// - single date: `"2026-04-01"` (midnight to midnight in `tz`)
    /// - date range: `"2026-04-01,2026-05-01"` — note the end date is **exclusive**, so this is
    ///   the whole of April
    /// - DateTime range: `"2026-04-01T10:00:00Z,2026-04-30T12:30:00+02:00"`
    ///
    /// Both `,` (canonical, as emitted by [`to_str`](Span::to_str)) and `..` are accepted as the
    /// range separator. Timestamps may be RFC3339 (with a `Z` or a numeric offset, which are
    /// honored and converted into `tz`) or a local `YYYY-MM-DDThh:mm:ss[.fff]` (interpreted in
    /// `tz`).
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str, tz: Tz) -> Option<Self> {
        let s = s.trim();

        if let Some(mode) = SpanMode::from_str(s)
            && mode.is_rel()
        {
            return Some(Span::make_rel(mode, tz));
        }

        if let Some((start_str, end_str)) = s
            .split_once(',')
            .or_else(|| s.split_once(".."))
            .map(|(a, b)| (a.trim(), b.trim()))
        {
            let start = parse_date_time(start_str, tz)?;
            let end = parse_date_time(end_str, tz)?;
            return Span::make_abs(start, end).ok();
        }

        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()?;
        Some(Span::make_date(Date::from(date), tz))
    }
}

/// Parse one side of a span range: either a date (midnight in `tz`) or a timestamp.
fn parse_date_time(s: &str, tz: Tz) -> Option<DateTime> {
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(date_at_midnight(date, tz));
    }
    // RFC3339 carries its own offset (`Z` or numeric), which identifies an absolute instant;
    // re-express it in `tz`.
    if let Ok(dt) = chrono::DateTime::<FixedOffset>::parse_from_rfc3339(s) {
        return Some(DateTime::from(dt.with_timezone(&tz)));
    }
    // Otherwise treat it as a local time in `tz`.
    for format in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M"] {
        if let Ok(local) = NaiveDateTime::parse_from_str(s, format) {
            return Some(local_to_date_time(local, tz));
        }
    }
    None
}

/// Resolve a local (wall clock) datetime in `tz` to an absolute `DateTime`.
///
/// DST makes this partial: a local time can be ambiguous (the clocks went back, so it happens
/// twice) or non-existent (the clocks went forward, skipping it). The earlier instant is used
/// when ambiguous; when the time falls in a gap, the first valid instant after the gap is used.
fn local_to_date_time(local: NaiveDateTime, tz: Tz) -> DateTime {
    match tz.from_local_datetime(&local) {
        LocalResult::Single(dt) => DateTime::from(dt),
        LocalResult::Ambiguous(earlier, _) => DateTime::from(earlier),
        LocalResult::None => {
            // Step forward until the local time exists again. Transitions are aligned to
            // whole minutes, and no real gap is longer than a few hours, but scan a full day
            // so this always terminates with a valid instant.
            let mut probe = local;
            for _ in 0..(24 * 60) {
                probe += TimeDelta::minutes(1);
                match tz.from_local_datetime(&probe) {
                    LocalResult::Single(dt) => return DateTime::from(dt),
                    LocalResult::Ambiguous(earlier, _) => return DateTime::from(earlier),
                    LocalResult::None => {}
                }
            }
            DateTime::from(local.and_utc().with_timezone(&tz))
        }
    }
}

/// Create a `DateTime` at midnight for a given date in a timezone.
///
/// In zones where DST starts at midnight, that midnight does not exist; see
/// [`local_to_date_time`] for how the gap is resolved.
fn date_at_midnight(date: NaiveDate, tz: Tz) -> DateTime {
    local_to_date_time(date.and_time(NaiveTime::MIN), tz)
}

/// Check if a DateTime is at local midnight.
fn is_midnight(dt: &DateTime) -> bool {
    dt.time() == NaiveTime::MIN
}

/// The [`START_OF_WEEK`] on or before `date`.
fn start_of_week(date: NaiveDate) -> NaiveDate {
    let days = (date.weekday().num_days_from_sunday() as i32
        - START_OF_WEEK.num_days_from_sunday() as i32)
        .rem_euclid(7);
    sub_days(date, days as u64)
}

impl From<&Span> for DateSpan {
    fn from(span: &Span) -> Self {
        span.to_date_span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Timelike};

    const LONDON: Tz = Tz::Europe__London;
    /// Cairo's DST starts at midnight, so some dates have no local midnight at all.
    const CAIRO: Tz = Tz::Africa__Cairo;

    fn d(y: i32, m: u32, day: u32) -> Date {
        Date::from(NaiveDate::from_ymd_opt(y, m, day).unwrap())
    }

    fn nd(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn span(s: &str, tz: Tz) -> Span {
        Span::from_str(s, tz).expect("span parses")
    }

    #[test]
    fn make_abs_creates_absolute_span() {
        let start = date_at_midnight(nd(2026, 4, 1), Tz::UTC);
        let end = date_at_midnight(nd(2026, 4, 30), Tz::UTC);
        let span = Span::make_abs(start, end).unwrap();
        assert_eq!(span.mode(), SpanMode::Abs);
        assert!(span.aligns_to_dates());
    }

    #[test]
    fn make_abs_rejects_mismatched_timezones_and_reversed_range() {
        let start = date_at_midnight(nd(2026, 4, 1), Tz::UTC);
        let end_other_tz = date_at_midnight(nd(2026, 4, 30), LONDON);
        assert!(Span::make_abs(start, end_other_tz).is_err());

        let later = date_at_midnight(nd(2026, 4, 30), Tz::UTC);
        assert!(Span::make_abs(later, start).is_err());
    }

    #[test]
    fn make_date_creates_midnight_to_midnight() {
        let span = Span::make_date(d(2026, 4, 15), Tz::UTC);
        assert_eq!(span.start().date_naive(), nd(2026, 4, 15));
        assert_eq!(span.end().date_naive(), nd(2026, 4, 16));
        assert_eq!(span.mode(), SpanMode::Abs);
        assert!(span.aligns_to_dates());
    }

    #[test]
    fn contains_inclusive_start_exclusive_end() {
        let span = span("2026-04-01,2026-04-30", Tz::UTC);
        assert!(span.contains(span.start()));
        assert!(!span.contains(span.end()));
    }

    #[test]
    fn to_date_span_converts_correctly() {
        // Span end is exclusive (May 1), so DateSpan end should be Apr 30
        let ds = span("2026-04-01,2026-05-01", Tz::UTC).to_date_span();
        assert_eq!(ds.start, d(2026, 4, 1));
        assert_eq!(ds.end, d(2026, 4, 30));
    }

    #[test]
    fn from_str_parses_mode_keyword() {
        assert!(span("today", Tz::UTC).mode().is_rel());
    }

    #[test]
    fn from_str_parses_date_range_with_either_separator() {
        for text in ["2026-04-01,2026-04-30", "2026-04-01..2026-04-30"] {
            let span = span(text, Tz::UTC);
            assert_eq!(span.mode(), SpanMode::Abs);
            assert!(span.aligns_to_dates());
            assert_eq!(span.num_days(), 29);
        }
    }

    #[test]
    fn num_days_for_month_span() {
        assert_eq!(span("2026-04-01,2026-05-01", Tz::UTC).num_days(), 30);
    }

    #[test]
    fn alignment_predicates() {
        let april = span("2026-04-01,2026-05-01", Tz::UTC);
        assert!(!april.aligns_to_day());
        assert!(april.aligns_to_month());
        assert!(!april.aligns_to_quarter());
        assert!(!april.aligns_to_week());
        assert!(!april.aligns_to_year());

        let q2 = span("2026-04-01,2026-07-01", Tz::UTC);
        assert!(q2.aligns_to_quarter());
        assert!(!q2.aligns_to_month());
        assert_eq!(q2.quarter(), 2);

        assert!(span("2026-01-01,2027-01-01", Tz::UTC).aligns_to_year());
        // 2026-04-05 is a Sunday
        assert!(span("2026-04-05,2026-04-12", Tz::UTC).aligns_to_week());
        assert!(span("2026-04-05", Tz::UTC).aligns_to_day());
    }

    #[test]
    fn dst_day_counts_as_one_calendar_day() {
        // 2026-03-29 is the 23-hour spring-forward day in London.
        let short_day = span("2026-03-29", LONDON);
        assert_eq!(short_day.num_days(), 1);
        assert!(short_day.aligns_to_day());
        let ds = short_day.to_date_span();
        assert_eq!(ds.start, d(2026, 3, 29));
        assert_eq!(ds.end, d(2026, 3, 29));
        assert_eq!(short_day.dis(), "29-Mar-2026..29-Mar-2026");

        // 2026-10-25 is the 25-hour autumn fall-back day in London.
        let long_day = span("2026-10-25", LONDON);
        assert_eq!(long_day.num_days(), 1);
        assert!(long_day.aligns_to_day());
    }

    #[test]
    fn month_spanning_dst_has_correct_day_count_and_navigation() {
        let march = span("2026-03-01,2026-04-01", LONDON);
        assert_eq!(march.num_days(), 31);
        assert!(march.aligns_to_month());
        assert_eq!(march.dis(), "01-Mar-2026..31-Mar-2026");

        let february = march.prev();
        assert_eq!(february.start().date_naive(), nd(2026, 2, 1));
        assert_eq!(february.end().date_naive(), nd(2026, 3, 1));
        assert_eq!(february.num_days(), 28);

        let april = march.next();
        assert_eq!(april.start().date_naive(), nd(2026, 4, 1));
        assert_eq!(april.end().date_naive(), nd(2026, 5, 1));
    }

    #[test]
    fn midnight_gap_timezone_keeps_span_timezone() {
        // Cairo moves the clocks forward at midnight on 2026-04-24, so that midnight is skipped.
        let span = span("2026-04-24", CAIRO);
        assert_eq!(span.tz(), CAIRO);
        assert_eq!(span.start().timezone(), CAIRO);
        assert_eq!(span.end().timezone(), CAIRO);
        // Start is pushed to the first instant that exists after the gap.
        assert_eq!(span.start().date_naive(), nd(2026, 4, 24));
        assert_eq!(span.start().hour(), 1);
        assert_eq!(span.num_days(), 1);
    }

    #[test]
    fn rfc3339_offsets_are_honored() {
        // 10:00 UTC is 11:00 in London during BST.
        let utc = span("2026-07-01T10:00:00Z,2026-07-01T12:00:00Z", LONDON);
        assert_eq!(utc.start().hour(), 11);
        assert_eq!(utc.start().naive_utc().hour(), 10);
        assert!(!utc.aligns_to_dates());

        // A local time with no offset is interpreted in the span's timezone.
        let local = span("2026-07-01T10:00:00,2026-07-01T12:00:00", LONDON);
        assert_eq!(local.start().hour(), 10);
        assert_eq!(local.start().naive_utc().hour(), 9);
    }

    #[test]
    fn to_str_round_trips() {
        for (text, tz) in [
            ("today", LONDON),
            ("thisMonth", LONDON),
            ("2026-03-29", LONDON),
            ("2026-03-01,2026-04-01", LONDON),
            ("2026-07-01T10:00:00Z,2026-07-03T12:30:00Z", LONDON),
        ] {
            let original = span(text, tz);
            let encoded = original.to_str();
            let parsed = Span::from_str(&encoded, tz)
                .unwrap_or_else(|| panic!("{encoded} (from {text}) should re-parse"));
            assert_eq!(parsed, original, "round trip of {text} via {encoded}");
        }
    }

    #[test]
    fn to_time_zone_recomputes_alignment() {
        let april = span("2026-04-01,2026-05-01", LONDON);
        let ny = april.to_time_zone(Tz::America__New_York);
        assert_eq!(ny.tz(), Tz::America__New_York);
        // Same instants, but no longer midnight-aligned in New York.
        assert_eq!(*ny.start(), *april.start());
        assert!(!ny.aligns_to_dates());
        assert!(!ny.aligns_to_month());
        assert_eq!(ny.mode(), SpanMode::Abs);
    }

    #[test]
    fn relative_modes_resolve_against_today() {
        let today = nd(2026, 4, 15); // a Wednesday
        let cases = [
            (SpanMode::Today, nd(2026, 4, 15), nd(2026, 4, 16)),
            (SpanMode::Yesterday, nd(2026, 4, 14), nd(2026, 4, 15)),
            (SpanMode::ThisWeek, nd(2026, 4, 12), nd(2026, 4, 19)),
            (SpanMode::LastWeek, nd(2026, 4, 5), nd(2026, 4, 12)),
            (SpanMode::PastWeek, nd(2026, 4, 8), nd(2026, 4, 15)),
            (SpanMode::ThisMonth, nd(2026, 4, 1), nd(2026, 5, 1)),
            (SpanMode::LastMonth, nd(2026, 3, 1), nd(2026, 4, 1)),
            (SpanMode::PastMonth, nd(2026, 3, 18), nd(2026, 4, 15)),
            (SpanMode::ThisQuarter, nd(2026, 4, 1), nd(2026, 7, 1)),
            (SpanMode::LastQuarter, nd(2026, 1, 1), nd(2026, 4, 1)),
            (SpanMode::ThisYear, nd(2026, 1, 1), nd(2027, 1, 1)),
            (SpanMode::LastYear, nd(2025, 1, 1), nd(2026, 1, 1)),
        ];
        for (mode, start, end) in cases {
            let span = Span::do_make_rel(mode, today, LONDON);
            assert_eq!(span.start().date_naive(), start, "{} start", mode.dis());
            assert_eq!(span.end().date_naive(), end, "{} end", mode.dis());
            assert_eq!(span.mode(), mode);
        }
    }

    #[test]
    fn each_day_covers_every_calendar_day_across_dst() {
        let mut days = 0;
        span("2026-03-01,2026-04-01", LONDON).each_day(|_, _| days += 1);
        assert_eq!(days, 31);
    }

    #[test]
    fn non_aligned_span_uses_both_local_dates() {
        let span = span("2026-07-01T10:00:00Z,2026-07-03T12:30:00Z", Tz::UTC);
        let ds = span.to_date_span();
        assert_eq!(ds.start, d(2026, 7, 1));
        assert_eq!(ds.end, d(2026, 7, 3));
        assert_eq!(span.dis(), "01-Jul-2026 10:00..03-Jul-2026 12:30");
    }

    #[test]
    fn prev_and_next_of_relative_modes_stay_relative() {
        assert_eq!(
            Span::make_rel(SpanMode::Today, LONDON).prev().mode(),
            SpanMode::Yesterday
        );
        assert_eq!(
            Span::make_rel(SpanMode::LastMonth, LONDON).next().mode(),
            SpanMode::ThisMonth
        );
    }
}
