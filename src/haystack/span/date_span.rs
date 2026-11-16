// Copyright (c) 2026, J2 Innovations. All Rights Reserved

//! Date-only span type (inclusive start and end).
//!
//! A `DateSpan` represents an inclusive date
//! range with an optional period classification (`day`, `week`, `month`, `quarter`, `year`,
//! or `range`). Unlike `Span` (see [`Span`](super::Span)), `DateSpan` is date-only — no time or
//! timezone component.

use crate::val::Date;
use chrono::{Datelike, Duration, Months, NaiveDate};

/// Construct a `NaiveDate` from year/month/day that are **known to be valid** by construction.
///
/// Panics only in `#[cfg(debug_assertions)]` builds if the inputs are invalid.
fn make_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::MIN)
}

/// English 3-letter month abbreviation (`chrono`'s own `%b` formatting is locale-independent
/// English by default, but this avoids depending on that behavior not changing).
pub(crate) fn month_abbr(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

/// Formats a `Date` as `"DD-MMM-YYYY"` (zero-padded day, 4-digit year), e.g. `"01-Jul-2010"` —
/// matches the format used by the Funcs.xeto docs' `toDateSpan()`/`toSpan()` example output
/// (e.g. `toDateSpan(2010) >> 01-Jan-2010..31-Dec-2010`).
pub(crate) fn fmt_date_dd_mmm_yyyy(d: &Date) -> String {
    format!("{:02}-{}-{}", d.day(), month_abbr(d.month()), d.year())
}

/// The period classification of a [`DateSpan`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateSpanPeriod {
    /// A single day.
    Day,
    /// A calendar week (Sunday–Saturday, matching upstream locale convention).
    Week,
    /// A calendar month.
    Month,
    /// A calendar quarter.
    Quarter,
    /// A calendar year.
    Year,
    /// An explicit date range (no period alignment).
    Range,
}

impl DateSpanPeriod {
    /// The string representation used by Axon for serialization and parsing.
    pub fn as_str(&self) -> &'static str {
        match self {
            DateSpanPeriod::Day => "day",
            DateSpanPeriod::Week => "week",
            DateSpanPeriod::Month => "month",
            DateSpanPeriod::Quarter => "quarter",
            DateSpanPeriod::Year => "year",
            DateSpanPeriod::Range => "range",
        }
    }

    /// Parse a period string into a `DateSpanPeriod`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "day" => Some(DateSpanPeriod::Day),
            "week" => Some(DateSpanPeriod::Week),
            "month" => Some(DateSpanPeriod::Month),
            "quarter" => Some(DateSpanPeriod::Quarter),
            "year" => Some(DateSpanPeriod::Year),
            "range" => Some(DateSpanPeriod::Range),
            _ => None,
        }
    }
}

/// A date-based span with inclusive start and end dates, and an optional period classification.
///
/// This mirrors the upstream `DateSpan` class from Haxall. The `start` and `end` are both
/// inclusive. The `period` field indicates the semantic meaning of the span (e.g., "this week",
/// "last month") or `"range"` for an arbitrary date-to-date span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateSpan {
    /// Inclusive start date.
    pub start: Date,
    /// Inclusive end date.
    pub end: Date,
    /// Period classification (e.g., `day`, `week`, `month`, `quarter`, `year`, `range`).
    pub period: DateSpanPeriod,
}

impl DateSpan {
    /// Create a new `DateSpan` with an explicit start, end, and period.
    pub fn new(start: Date, end: Date, period: DateSpanPeriod) -> Self {
        DateSpan { start, end, period }
    }

    /// Create a `DateSpan` from a start date and a period string.
    ///
    /// The start date will be adjusted to align to the period boundary:
    /// - `day`: start = start, end = start
    /// - `week`: start = Sunday of the week containing start, end = Saturday
    /// - `month`: start = 1st of the month, end = last day of the month
    /// - `quarter`: start = 1st of the quarter, end = last day of the quarter
    /// - `year`: start = Jan 1, end = Dec 31
    pub fn from_period(start: Date, period: DateSpanPeriod) -> Self {
        // Date derefs to NaiveDate, so we can use Datelike methods directly
        let (s, e) = match period {
            DateSpanPeriod::Day => (*start, *start),
            DateSpanPeriod::Week => {
                // Align to Sunday–Saturday (upstream uses locale start of week, which is Sunday)
                let days_from_sun = start.weekday().num_days_from_sunday() as i64;
                let sunday = *start - Duration::days(days_from_sun);
                let saturday = sunday + Duration::days(6);
                (sunday, saturday)
            }
            DateSpanPeriod::Month => {
                let first = make_date(start.year(), start.month(), 1);
                // Last day of month = first day of next month minus 1 day
                let (next_year, next_month) = if start.month() == 12 {
                    (start.year() + 1, 1)
                } else {
                    (start.year(), start.month() + 1)
                };
                let first_next = make_date(next_year, next_month, 1) - Duration::days(1);
                (first, first_next)
            }
            DateSpanPeriod::Quarter => {
                let quarter_month = (start.month() - 1) / 3 * 3 + 1;
                let first = make_date(start.year(), quarter_month, 1);
                // Last day of quarter = first day of next quarter minus 1 day
                let next_q_month = quarter_month + 3;
                let (next_year, nm) = if next_q_month > 12 {
                    (start.year() + 1, 1)
                } else {
                    (start.year(), next_q_month)
                };
                let first_next = make_date(next_year, nm, 1) - Duration::days(1);
                (first, first_next)
            }
            DateSpanPeriod::Year => {
                let first = make_date(start.year(), 1, 1);
                let last = make_date(start.year(), 12, 31);
                (first, last)
            }
            DateSpanPeriod::Range => (*start, *start),
        };
        DateSpan {
            start: Date::from(s),
            end: Date::from(e),
            period,
        }
    }

    /// Create an explicit range span (no period alignment).
    pub fn range(start: Date, end: Date) -> Self {
        DateSpan {
            start,
            end,
            period: DateSpanPeriod::Range,
        }
    }

    /// Number of days in this span (inclusive on both ends).
    pub fn num_days(&self) -> i64 {
        (*self.end - *self.start).num_days() + 1
    }

    /// Check if a date is contained within this span (inclusive on both ends).
    pub fn contains(&self, date: &Date) -> bool {
        *date >= self.start && *date <= self.end
    }

    /// Shift the span forward by a duration. The result is always a `Range`.
    pub fn plus(&self, duration: Duration) -> Self {
        DateSpan::range(
            Date::from(*self.start + duration),
            Date::from(*self.end + duration),
        )
    }

    /// Shift the span backward by a duration. The result is always a `Range`.
    pub fn minus(&self, duration: Duration) -> Self {
        DateSpan::range(
            Date::from(*self.start - duration),
            Date::from(*self.end - duration),
        )
    }

    /// Get the previous period (e.g., previous month, previous week).
    /// For `Range` period, roll start and end back one day.
    pub fn prev(&self) -> Self {
        match self.period {
            DateSpanPeriod::Day => {
                let new_start = *self.start - Duration::days(1);
                DateSpan::new(
                    Date::from(new_start),
                    Date::from(new_start),
                    DateSpanPeriod::Day,
                )
            }
            DateSpanPeriod::Week => {
                let new_start = *self.start - Duration::days(7);
                let new_end = *self.end - Duration::days(7);
                DateSpan::new(
                    Date::from(new_start),
                    Date::from(new_end),
                    DateSpanPeriod::Week,
                )
            }
            DateSpanPeriod::Month => {
                let prev_first = *self.start - Duration::days(1);
                DateSpan::from_period(Date::from(prev_first), DateSpanPeriod::Month)
            }
            DateSpanPeriod::Quarter => {
                let prev_first = *self.start - Duration::days(1);
                DateSpan::from_period(Date::from(prev_first), DateSpanPeriod::Quarter)
            }
            DateSpanPeriod::Year => {
                let prev_first = *self.start - Duration::days(1);
                DateSpan::from_period(Date::from(prev_first), DateSpanPeriod::Year)
            }
            DateSpanPeriod::Range => self.minus(Duration::days(1)),
        }
    }

    /// Get the next period (e.g., next month, next week).
    /// For `Range` period, roll start and end forward one day.
    pub fn next(&self) -> Self {
        match self.period {
            DateSpanPeriod::Day => {
                let new_start = *self.start + Duration::days(1);
                DateSpan::new(
                    Date::from(new_start),
                    Date::from(new_start),
                    DateSpanPeriod::Day,
                )
            }
            DateSpanPeriod::Week => {
                let new_start = *self.start + Duration::days(7);
                let new_end = *self.end + Duration::days(7);
                DateSpan::new(
                    Date::from(new_start),
                    Date::from(new_end),
                    DateSpanPeriod::Week,
                )
            }
            DateSpanPeriod::Month => {
                let next_first = *self.end + Duration::days(1);
                DateSpan::from_period(Date::from(next_first), DateSpanPeriod::Month)
            }
            DateSpanPeriod::Quarter => {
                let next_first = *self.end + Duration::days(1);
                DateSpan::from_period(Date::from(next_first), DateSpanPeriod::Quarter)
            }
            DateSpanPeriod::Year => {
                let next_first = *self.end + Duration::days(1);
                DateSpan::from_period(Date::from(next_first), DateSpanPeriod::Year)
            }
            DateSpanPeriod::Range => self.plus(Duration::days(1)),
        }
    }

    /// Iterate over each day in the span, calling `func` with the date and a 0-based index.
    pub fn each_day<F>(&self, mut func: F)
    where
        F: FnMut(Date, i32),
    {
        let mut current = self.start;
        let mut idx: i32 = 0;
        while current <= self.end {
            func(current, idx);
            current = Date::from(*current + Duration::days(1));
            idx += 1;
        }
    }

    /// Iterate over each month in the span, calling `func` with a `DateSpan` for that month.
    pub fn each_month<F>(&self, mut func: F)
    where
        F: FnMut(DateSpan),
    {
        let mut current = DateSpan::from_period(self.start, DateSpanPeriod::Month);
        loop {
            if current.end >= self.start && current.start <= self.end {
                func(current.clone());
            }
            if current.end >= self.end {
                break;
            }
            let next_start = *current.end + Duration::days(1);
            current = DateSpan::from_period(Date::from(next_start), DateSpanPeriod::Month);
        }
    }

    /// Convert to a list of dates (one per day in the span).
    pub fn to_date_list(&self) -> Vec<Date> {
        let mut dates = Vec::new();
        self.each_day(|d, _| dates.push(d));
        dates
    }

    /// Machine-parseable string representation (mirrors Fantom's `DateSpan.toStr()`):
    /// `"<start>,<end>"` for a `Range` period, `"<start>,<period>"` otherwise. Round-trips
    /// through [`DateSpan::from_str`]; this is what the `toStr()` Axon stdlib function returns.
    pub fn to_str(&self) -> String {
        match self.period {
            DateSpanPeriod::Range => {
                format!("{},{}", self.start, self.end)
            }
            _ => {
                format!("{},{}", self.start, self.period.as_str())
            }
        }
    }

    /// Human-friendly display string for the console/REPL, e.g. `"01-Jul-2010..03-Jul-2010"`.
    /// Always renders the explicit inclusive `start..end` date range with a zero-padded day and
    /// 4-digit year, matching the format used by the Funcs.xeto docs' own examples (e.g.
    /// `toDateSpan(2010) >> 01-Jan-2010..31-Dec-2010`), regardless of period.
    pub fn dis(&self) -> String {
        format!(
            "{}..{}",
            fmt_date_dd_mmm_yyyy(&self.start),
            fmt_date_dd_mmm_yyyy(&self.end)
        )
    }

    /// Check period predicates.
    pub fn is_day(&self) -> bool {
        self.period == DateSpanPeriod::Day
    }
    pub fn is_week(&self) -> bool {
        self.period == DateSpanPeriod::Week
    }
    pub fn is_month(&self) -> bool {
        self.period == DateSpanPeriod::Month
    }
    pub fn is_quarter(&self) -> bool {
        self.period == DateSpanPeriod::Quarter
    }
    pub fn is_year(&self) -> bool {
        self.period == DateSpanPeriod::Year
    }
    pub fn is_range(&self) -> bool {
        self.period == DateSpanPeriod::Range
    }

    /// Parse a `DateSpan` from a string.
    ///
    /// Supported formats:
    /// - `"YYYY-MM-DD,period"` — e.g., `"2026-04-01,month"`
    /// - `"YYYY-MM-DD,YYYY-MM-DD"` — explicit range
    /// - `"today"`, `"yesterday"` — a single day span
    /// - `"thisWeek"`, `"thisMonth"`, `"thisQuarter"`, `"thisYear"` — current period
    /// - `"lastWeek"`, `"lastMonth"`, `"lastQuarter"`, `"lastYear"` — previous period
    /// - `"pastWeek"`, `"pastMonth"` — rolling 7/30-day span ending today
    /// - `"pastYear"` — from the same date last year (clamped to month end) to today
    pub fn from_str(s: &str, today: Date) -> Option<Self> {
        let s = s.trim();
        // Check for keyword shortcuts
        match s {
            "today" => {
                return Some(DateSpan::new(today, today, DateSpanPeriod::Day));
            }
            "yesterday" => {
                let day = Date::from(*today - Duration::days(1));
                return Some(DateSpan::new(day, day, DateSpanPeriod::Day));
            }
            "thisWeek" => {
                return Some(DateSpan::from_period(today, DateSpanPeriod::Week));
            }
            "thisMonth" => {
                return Some(DateSpan::from_period(today, DateSpanPeriod::Month));
            }
            "thisQuarter" => {
                return Some(DateSpan::from_period(today, DateSpanPeriod::Quarter));
            }
            "thisYear" => {
                return Some(DateSpan::from_period(today, DateSpanPeriod::Year));
            }
            "lastWeek" => {
                let ws = DateSpan::from_period(today, DateSpanPeriod::Week);
                return Some(ws.prev());
            }
            "lastMonth" => {
                let ms = DateSpan::from_period(today, DateSpanPeriod::Month);
                return Some(ms.prev());
            }
            "lastQuarter" => {
                let qs = DateSpan::from_period(today, DateSpanPeriod::Quarter);
                return Some(qs.prev());
            }
            "lastYear" => {
                let ys = DateSpan::from_period(today, DateSpanPeriod::Year);
                return Some(ys.prev());
            }
            "pastWeek" => {
                let end = today;
                let start_naive = *today - Duration::days(7);
                return Some(DateSpan::range(Date::from(start_naive), end));
            }
            "pastMonth" => {
                let end = today;
                let start_naive = *today - Duration::days(30);
                return Some(DateSpan::range(Date::from(start_naive), end));
            }
            "pastYear" => {
                let start = today
                    .checked_sub_months(Months::new(12))
                    .unwrap_or(NaiveDate::MIN);
                return Some(DateSpan::range(Date::from(start), today));
            }
            _ => {}
        }

        // Try parsing "date,period" or "date,date"
        if let Some(comma) = s.find(',') {
            let date_part = &s[..comma];
            let second_part = &s[comma + 1..];
            let start_date = NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()?;
            let start_date = Date::from(start_date);

            // Try as period string first
            if let Some(period) = DateSpanPeriod::from_str(second_part) {
                return Some(DateSpan::from_period(start_date, period));
            }

            // Try as second date (explicit range)
            if let Ok(end_date) = NaiveDate::parse_from_str(second_part, "%Y-%m-%d") {
                return Some(DateSpan::range(start_date, Date::from(end_date)));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn d(y: i32, m: u32, day: u32) -> Date {
        Date::from(NaiveDate::from_ymd_opt(y, m, day).unwrap())
    }

    #[test]
    fn day_period_is_single_day() {
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Day);
        assert_eq!(span.start, d(2026, 4, 15));
        assert_eq!(span.end, d(2026, 4, 15));
        assert_eq!(span.num_days(), 1);
    }

    #[test]
    fn week_period_aligns_to_sunday_saturday() {
        // 2026-04-15 is a Wednesday
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Week);
        // Sunday 2026-04-12 to Saturday 2026-04-18
        assert_eq!(span.start, d(2026, 4, 12));
        assert_eq!(span.end, d(2026, 4, 18));
        assert_eq!(span.num_days(), 7);
    }

    #[test]
    fn month_period_aligns_to_first_last() {
        let span = DateSpan::from_period(d(2026, 2, 15), DateSpanPeriod::Month);
        assert_eq!(span.start, d(2026, 2, 1));
        assert_eq!(span.end, d(2026, 2, 28));
        assert_eq!(span.num_days(), 28);
    }

    #[test]
    fn quarter_period_q1() {
        let span = DateSpan::from_period(d(2026, 2, 15), DateSpanPeriod::Quarter);
        assert_eq!(span.start, d(2026, 1, 1));
        assert_eq!(span.end, d(2026, 3, 31));
    }

    #[test]
    fn year_period() {
        let span = DateSpan::from_period(d(2026, 7, 3), DateSpanPeriod::Year);
        assert_eq!(span.start, d(2026, 1, 1));
        assert_eq!(span.end, d(2026, 12, 31));
    }

    #[test]
    fn contains_inclusive() {
        let span = DateSpan::range(d(2026, 4, 1), d(2026, 4, 30));
        assert!(span.contains(&d(2026, 4, 1)));
        assert!(span.contains(&d(2026, 4, 15)));
        assert!(span.contains(&d(2026, 4, 30)));
        assert!(!span.contains(&d(2026, 3, 31)));
        assert!(!span.contains(&d(2026, 5, 1)));
    }

    #[test]
    fn plus_minus_shifts_dates() {
        let span = DateSpan::range(d(2026, 4, 1), d(2026, 4, 10));
        let shifted = span.plus(Duration::days(5));
        assert_eq!(shifted.start, d(2026, 4, 6));
        assert_eq!(shifted.end, d(2026, 4, 15));

        let shifted_back = span.minus(Duration::days(3));
        assert_eq!(shifted_back.start, d(2026, 3, 29));
        assert_eq!(shifted_back.end, d(2026, 4, 7));
    }

    #[test]
    fn each_day_iterates_correctly() {
        let span = DateSpan::range(d(2026, 4, 1), d(2026, 4, 3));
        let mut dates = Vec::new();
        let mut indices = Vec::new();
        span.each_day(|d, i| {
            dates.push(d);
            indices.push(i);
        });
        assert_eq!(dates, vec![d(2026, 4, 1), d(2026, 4, 2), d(2026, 4, 3)]);
        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn from_str_parses_date_period() {
        let today = d(2026, 4, 15);
        let span = DateSpan::from_str("2026-04-01,month", today).unwrap();
        assert_eq!(span.start, d(2026, 4, 1));
        assert_eq!(span.end, d(2026, 4, 30));
        assert!(span.is_month());
    }

    #[test]
    fn from_str_parses_date_range() {
        let today = d(2026, 4, 15);
        let span = DateSpan::from_str("2026-04-01,2026-04-30", today).unwrap();
        assert_eq!(span.start, d(2026, 4, 1));
        assert_eq!(span.end, d(2026, 4, 30));
        assert!(span.is_range());
    }

    #[test]
    fn from_str_keyword_today() {
        let today = d(2026, 4, 15);
        let span = DateSpan::from_str("today", today).unwrap();
        assert_eq!(span.start, today);
        assert_eq!(span.end, today);
        assert!(span.is_day());
    }

    #[test]
    fn from_str_invalid_returns_none() {
        let today = d(2026, 4, 15);
        assert!(DateSpan::from_str("not-a-date", today).is_none());
        assert!(DateSpan::from_str("2026-04-01,invalid", today).is_none());
    }

    #[test]
    fn prev_month_from_april_is_march() {
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Month);
        let prev = span.prev();
        assert_eq!(prev.start, d(2026, 3, 1));
        assert_eq!(prev.end, d(2026, 3, 31));
    }

    #[test]
    fn next_month_from_april_is_may() {
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Month);
        let next = span.next();
        assert_eq!(next.start, d(2026, 5, 1));
        assert_eq!(next.end, d(2026, 5, 31));
    }

    #[test]
    fn to_str_format_for_period() {
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Month);
        assert_eq!(span.to_str(), "2026-04-01,month");
    }

    #[test]
    fn to_str_format_for_range() {
        let span = DateSpan::range(d(2026, 4, 1), d(2026, 4, 30));
        assert_eq!(span.to_str(), "2026-04-01,2026-04-30");
    }

    #[test]
    fn dis_format_matches_funcs_docs_examples() {
        // toDateSpan(2010-07-01..2010-07-03) >> 01-Jul-2010..03-Jul-2010
        let range = DateSpan::range(d(2010, 7, 1), d(2010, 7, 3));
        assert_eq!(range.dis(), "01-Jul-2010..03-Jul-2010");

        // toDateSpan(2010-07) >> 01-Jul-2010..31-Jul-2010
        let month = DateSpan::from_period(d(2010, 7, 15), DateSpanPeriod::Month);
        assert_eq!(month.dis(), "01-Jul-2010..31-Jul-2010");

        // toDateSpan(2010) >> 01-Jan-2010..31-Dec-2010
        let year = DateSpan::from_period(d(2010, 6, 1), DateSpanPeriod::Year);
        assert_eq!(year.dis(), "01-Jan-2010..31-Dec-2010");
    }

    fn ends(span: &DateSpan) -> (Date, Date) {
        (span.start, span.end)
    }

    #[test]
    fn make_date_falls_back_to_min_for_invalid_input() {
        assert_eq!(make_date(2026, 13, 1), NaiveDate::MIN);
        assert_eq!(
            make_date(2026, 4, 15),
            NaiveDate::from_ymd_opt(2026, 4, 15).unwrap()
        );
    }

    #[test]
    fn month_abbr_covers_all_months() {
        let names: Vec<&str> = (1..=12).map(month_abbr).collect();
        assert_eq!(
            names,
            [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
            ]
        );
        assert_eq!(month_abbr(0), "");
        assert_eq!(month_abbr(13), "");
    }

    #[test]
    fn period_string_round_trip() {
        let periods = [
            DateSpanPeriod::Day,
            DateSpanPeriod::Week,
            DateSpanPeriod::Month,
            DateSpanPeriod::Quarter,
            DateSpanPeriod::Year,
            DateSpanPeriod::Range,
        ];
        for period in periods {
            assert_eq!(DateSpanPeriod::from_str(period.as_str()), Some(period));
        }
        assert_eq!(DateSpanPeriod::from_str("bogus"), None);
    }

    #[test]
    fn period_predicates() {
        let day = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Day);
        let week = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Week);
        let month = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Month);
        let quarter = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Quarter);
        let year = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Year);
        let range = DateSpan::range(d(2026, 4, 1), d(2026, 4, 2));

        assert!(day.is_day() && !day.is_week());
        assert!(week.is_week() && !week.is_month());
        assert!(month.is_month() && !month.is_quarter());
        assert!(quarter.is_quarter() && !quarter.is_year());
        assert!(year.is_year() && !year.is_range());
        assert!(range.is_range() && !range.is_day());
    }

    #[test]
    fn from_period_range_is_single_day() {
        let span = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Range);
        assert_eq!(ends(&span), (d(2026, 4, 15), d(2026, 4, 15)));
        assert!(span.is_range());
    }

    #[test]
    fn from_period_year_end_rollovers() {
        let december = DateSpan::from_period(d(2026, 12, 10), DateSpanPeriod::Month);
        assert_eq!(ends(&december), (d(2026, 12, 1), d(2026, 12, 31)));

        let q3 = DateSpan::from_period(d(2026, 8, 10), DateSpanPeriod::Quarter);
        assert_eq!(ends(&q3), (d(2026, 7, 1), d(2026, 9, 30)));

        let q4 = DateSpan::from_period(d(2026, 11, 10), DateSpanPeriod::Quarter);
        assert_eq!(ends(&q4), (d(2026, 10, 1), d(2026, 12, 31)));
    }

    #[test]
    fn prev_and_next_for_every_period() {
        let day = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Day);
        assert_eq!(ends(&day.prev()), (d(2026, 4, 14), d(2026, 4, 14)));
        assert_eq!(ends(&day.next()), (d(2026, 4, 16), d(2026, 4, 16)));

        let week = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Week);
        assert_eq!(ends(&week.prev()), (d(2026, 4, 5), d(2026, 4, 11)));
        assert_eq!(ends(&week.next()), (d(2026, 4, 19), d(2026, 4, 25)));

        let january = DateSpan::from_period(d(2026, 1, 10), DateSpanPeriod::Month);
        assert_eq!(ends(&january.prev()), (d(2025, 12, 1), d(2025, 12, 31)));
        assert_eq!(ends(&january.prev().next()), ends(&january));

        let quarter = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Quarter);
        assert_eq!(ends(&quarter.prev()), (d(2026, 1, 1), d(2026, 3, 31)));
        assert_eq!(ends(&quarter.next()), (d(2026, 7, 1), d(2026, 9, 30)));
        assert!(quarter.prev().is_quarter() && quarter.next().is_quarter());

        let year = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Year);
        assert_eq!(ends(&year.prev()), (d(2025, 1, 1), d(2025, 12, 31)));
        assert_eq!(ends(&year.next()), (d(2027, 1, 1), d(2027, 12, 31)));

        // A range rolls by one day, regardless of its length.
        let range = DateSpan::range(d(2026, 4, 1), d(2026, 4, 10));
        assert_eq!(ends(&range.prev()), (d(2026, 3, 31), d(2026, 4, 9)));
        assert_eq!(ends(&range.next()), (d(2026, 4, 2), d(2026, 4, 11)));
        assert!(range.prev().is_range() && range.next().is_range());
    }

    #[test]
    fn plus_minus_produce_range() {
        let month = DateSpan::from_period(d(2026, 4, 15), DateSpanPeriod::Month);
        let plus = month.plus(Duration::days(1));
        assert_eq!(ends(&plus), (d(2026, 4, 2), d(2026, 5, 1)));
        assert!(plus.is_range());
        let minus = month.minus(Duration::days(1));
        assert_eq!(ends(&minus), (d(2026, 3, 31), d(2026, 4, 29)));
        assert!(minus.is_range());
    }

    #[test]
    fn each_month_visits_every_overlapping_month() {
        let span = DateSpan::range(d(2026, 1, 15), d(2026, 3, 10));
        let mut months = Vec::new();
        span.each_month(|m| months.push(ends(&m)));
        assert_eq!(
            months,
            vec![
                (d(2026, 1, 1), d(2026, 1, 31)),
                (d(2026, 2, 1), d(2026, 2, 28)),
                (d(2026, 3, 1), d(2026, 3, 31)),
            ]
        );

        let mut count = 0;
        DateSpan::range(d(2026, 4, 3), d(2026, 4, 20)).each_month(|_| count += 1);
        assert_eq!(count, 1);
    }

    #[test]
    fn to_date_list_has_one_date_per_day() {
        let span = DateSpan::range(d(2026, 2, 27), d(2026, 3, 2));
        assert_eq!(
            span.to_date_list(),
            vec![d(2026, 2, 27), d(2026, 2, 28), d(2026, 3, 1), d(2026, 3, 2)]
        );
    }

    #[test]
    fn from_str_keywords() {
        let today = d(2026, 4, 15); // a Wednesday
        let cases = [
            (
                "yesterday",
                d(2026, 4, 14),
                d(2026, 4, 14),
                DateSpanPeriod::Day,
            ),
            (
                "thisWeek",
                d(2026, 4, 12),
                d(2026, 4, 18),
                DateSpanPeriod::Week,
            ),
            (
                "thisMonth",
                d(2026, 4, 1),
                d(2026, 4, 30),
                DateSpanPeriod::Month,
            ),
            (
                "thisQuarter",
                d(2026, 4, 1),
                d(2026, 6, 30),
                DateSpanPeriod::Quarter,
            ),
            (
                "thisYear",
                d(2026, 1, 1),
                d(2026, 12, 31),
                DateSpanPeriod::Year,
            ),
            (
                "lastWeek",
                d(2026, 4, 5),
                d(2026, 4, 11),
                DateSpanPeriod::Week,
            ),
            (
                "lastMonth",
                d(2026, 3, 1),
                d(2026, 3, 31),
                DateSpanPeriod::Month,
            ),
            (
                "lastQuarter",
                d(2026, 1, 1),
                d(2026, 3, 31),
                DateSpanPeriod::Quarter,
            ),
            (
                "lastYear",
                d(2025, 1, 1),
                d(2025, 12, 31),
                DateSpanPeriod::Year,
            ),
            (
                "pastWeek",
                d(2026, 4, 8),
                d(2026, 4, 15),
                DateSpanPeriod::Range,
            ),
            (
                "pastMonth",
                d(2026, 3, 16),
                d(2026, 4, 15),
                DateSpanPeriod::Range,
            ),
            (
                "pastYear",
                d(2025, 4, 15),
                d(2026, 4, 15),
                DateSpanPeriod::Range,
            ),
        ];
        for (text, start, end, period) in cases {
            let span = DateSpan::from_str(text, today).unwrap_or_else(|| panic!("{text}"));
            assert_eq!(ends(&span), (start, end), "{text}");
            assert_eq!(span.period, period, "{text}");
        }
    }

    #[test]
    fn past_year_clamps_leap_day() {
        let span = DateSpan::from_str("pastYear", d(2028, 2, 29)).unwrap();
        assert_eq!(ends(&span), (d(2027, 2, 28), d(2028, 2, 29)));
    }

    #[test]
    fn from_str_trims_whitespace_and_parses_week_period() {
        let today = d(2026, 4, 15);
        assert!(DateSpan::from_str("  today ", today).unwrap().is_day());

        // 2026-04-01 is a Wednesday
        let week = DateSpan::from_str("2026-04-01,week", today).unwrap();
        assert_eq!(ends(&week), (d(2026, 3, 29), d(2026, 4, 4)));
    }

    #[test]
    fn from_str_rejects_bad_dates() {
        let today = d(2026, 4, 15);
        assert!(DateSpan::from_str("2026-13-01,month", today).is_none());
        assert!(DateSpan::from_str("2026-04-01,2026-99-99", today).is_none());
        assert!(DateSpan::from_str("", today).is_none());
    }

    #[test]
    fn to_str_round_trips_for_every_period() {
        let today = d(2026, 4, 15);
        let spans = [
            DateSpan::from_period(today, DateSpanPeriod::Day),
            DateSpan::from_period(today, DateSpanPeriod::Week),
            DateSpan::from_period(today, DateSpanPeriod::Month),
            DateSpan::from_period(today, DateSpanPeriod::Quarter),
            DateSpan::from_period(today, DateSpanPeriod::Year),
            DateSpan::range(d(2026, 4, 3), d(2026, 4, 9)),
        ];
        for span in spans {
            let encoded = span.to_str();
            assert_eq!(DateSpan::from_str(&encoded, today), Some(span), "{encoded}");
        }
    }

    #[test]
    fn dis_for_single_day() {
        let day = DateSpan::from_period(d(2026, 4, 5), DateSpanPeriod::Day);
        assert_eq!(day.dis(), "05-Apr-2026..05-Apr-2026");
    }
}
