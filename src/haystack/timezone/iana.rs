// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Timezone configured to work with the full IANA database
//! provided by chrono_tz.

use chrono::{
    DateTime as StdDateTime, FixedOffset, NaiveDateTime, Offset, TimeZone, Timelike, Utc,
};

use chrono_tz::{OffsetName, Tz, UTC};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use crate::timezone::fixed_timezone;

/// A compact datetime value backed by the IANA timezone database.
///
/// `chrono::DateTime<chrono_tz::Tz>` caches a fully resolved offset
/// (`chrono_tz::TzOffset`, which itself carries an optional static DST/STD
/// name) alongside the naive datetime, making it 48 bytes. This type instead
/// stores only the naive UTC datetime (12 bytes) and the timezone id
/// (`chrono_tz::Tz`, 2 bytes), and resolves the offset on demand, keeping the
/// overall type (and therefore `Value::DateTime`) small.
#[derive(Copy, Clone, Debug)]
pub struct DateTimeType {
    utc: NaiveDateTime,
    tz: Tz,
}

impl DateTimeType {
    /// The naive (timezone-less) UTC datetime.
    pub fn naive_utc(&self) -> NaiveDateTime {
        self.utc
    }

    /// The naive (timezone-less) local datetime.
    pub fn naive_local(&self) -> NaiveDateTime {
        self.utc + self.offset().fix()
    }

    /// The offset resolved for this datetime's instant, in this timezone.
    pub fn offset(&self) -> <Tz as TimeZone>::Offset {
        self.tz.offset_from_utc_datetime(&self.utc)
    }

    /// This datetime's timezone.
    pub fn timezone(&self) -> Tz {
        self.tz
    }

    /// The same instant, re-expressed with a fixed UTC offset.
    pub fn to_fixed_offset(&self) -> StdDateTime<FixedOffset> {
        self.utc.and_utc().fixed_offset()
    }

    /// The same instant, re-expressed with the given fixed offset.
    pub fn with_timezone(&self, tz: &FixedOffset) -> StdDateTime<FixedOffset> {
        self.to_fixed_offset().with_timezone(tz)
    }

    /// See `chrono::DateTime::to_rfc3339`.
    pub fn to_rfc3339(&self) -> String {
        StdDateTime::<Tz>::from_naive_utc_and_offset(self.utc, self.offset()).to_rfc3339()
    }

    /// See `chrono::DateTime::to_rfc3339_opts`.
    pub fn to_rfc3339_opts(&self, secform: chrono::SecondsFormat, use_z: bool) -> String {
        StdDateTime::<Tz>::from_naive_utc_and_offset(self.utc, self.offset())
            .to_rfc3339_opts(secform, use_z)
    }

    /// The Unix timestamp, in seconds.
    pub fn timestamp(&self) -> i64 {
        self.utc.and_utc().timestamp()
    }

    /// The sub-second nanosecond component of this datetime's timestamp.
    pub fn timestamp_subsec_nanos(&self) -> u32 {
        self.utc.and_utc().timestamp_subsec_nanos()
    }

    /// The hour of this datetime's local time (0-23).
    pub fn hour(&self) -> u32 {
        self.naive_local().hour()
    }

    /// The minute of this datetime's local time.
    pub fn minute(&self) -> u32 {
        self.naive_local().minute()
    }

    /// The second of this datetime's local time.
    pub fn second(&self) -> u32 {
        self.naive_local().second()
    }

    /// The nanosecond component of this datetime's local time.
    pub fn nanosecond(&self) -> u32 {
        self.naive_local().nanosecond()
    }
}

/// Builds a compact `DateTimeType` from a fully resolved `chrono::DateTime<Tz>`.
impl From<StdDateTime<Tz>> for DateTimeType {
    fn from(full: StdDateTime<Tz>) -> Self {
        DateTimeType {
            utc: full.naive_utc(),
            tz: full.timezone(),
        }
    }
}

/// Equality, ordering and hashing are based purely on the UTC instant,
/// matching `chrono::DateTime`'s own semantics (the timezone is ignored).
impl PartialEq for DateTimeType {
    fn eq(&self, other: &Self) -> bool {
        self.utc == other.utc
    }
}

impl Eq for DateTimeType {}

impl PartialOrd for DateTimeType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTimeType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.utc.cmp(&other.utc)
    }
}

impl Hash for DateTimeType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.utc.hash(state)
    }
}

pub fn make_date_time(date: StdDateTime<FixedOffset>) -> Result<DateTimeType, String> {
    use chrono::LocalResult;
    if let Ok(tz) = find_timezone(&fixed_timezone(&date.offset().to_string())) {
        Ok(match tz.from_local_datetime(&date.naive_local()) {
            LocalResult::Single(val) => val.into(),
            LocalResult::Ambiguous(v1, _) => v1.into(),
            LocalResult::None => return Err(format!("Can't create datetime with timezone {tz}")),
        })
    } else {
        Err("Invalid timezone".into())
    }
}

/// Constructs a datetime with the timezone
pub fn make_date_time_with_tz(
    datetime: &StdDateTime<FixedOffset>,
    tz: &str,
) -> Result<DateTimeType, String> {
    if let Ok(tz) = find_timezone(tz) {
        Ok(datetime.with_timezone(&tz).into())
    } else {
        Err(format!("Can't create datetime with timezone {tz}"))
    }
}

pub fn utc_now() -> DateTimeType {
    Utc::now().with_timezone(&UTC).into()
}

pub fn is_utc(date: &DateTimeType) -> bool {
    date.timezone() == UTC
}

pub fn timezone_short_name(date: &DateTimeType) -> String {
    let offset = date.offset();
    let tz_id = offset.tz_id();

    tz_id[tz_id.find('/').map_or(0, |v| v + 1)..].to_string()
}

static PREFIXES: [&str; 18] = [
    "Africa",
    "America",
    "Asia",
    "Atlantic",
    "Australia",
    "Brazil",
    "Canada",
    "Chile",
    "Etc",
    "Europe",
    "Indian",
    "Mexico",
    "Pacific",
    "US",
    "America/Argentina",
    "America/Indiana",
    "America/Kentucky",
    "America/North_Dakota",
];

fn find_timezone(name: &str) -> Result<Tz, String> {
    name.parse().or_else(|err: chrono_tz::ParseError| {
        PREFIXES
            .into_iter()
            .find_map(|prefix| format!("{prefix}/{name}").parse().ok())
            .ok_or(err.to_string())
    })
}
