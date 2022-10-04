// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Timezone configured to work with the full IANA database
//! provided by chrono_tz.

use chrono::{DateTime as StdDateTime, FixedOffset, TimeZone, Utc};

use chrono_tz::{OffsetName, Tz, UTC};

use crate::timezone::fixed_timezone;

/// DateTime type that supports timezones
pub type DateTimeType = StdDateTime<Tz>;

pub fn make_date_time(date: StdDateTime<FixedOffset>) -> Result<DateTimeType, String> {
    use chrono::LocalResult;
    if let Ok(tz) = find_timezone(&fixed_timezone(&date.offset().to_string())) {
        Ok(match tz.from_local_datetime(&date.naive_local()) {
            LocalResult::Single(val) => val,
            LocalResult::Ambiguous(v1, _) => v1,
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
        Ok(datetime.with_timezone(&tz))
    } else {
        Err(format!("Can't create datetime with timezone {tz}"))
    }
}

pub fn utc_now() -> DateTimeType {
    Utc::now().with_timezone(&UTC)
}

pub fn is_utc(date: &DateTimeType) -> bool {
    date.timezone() == UTC
}

pub fn timezone_short_name(date: &DateTimeType) -> String {
    let tz_id = date.offset().tz_id();

    tz_id[tz_id.find('/').map_or(0, |v| v + 1)..].to_string()
}

fn find_timezone(name: &str) -> Result<Tz, String> {
    match name.parse() {
        Ok(tz) => Ok(tz),
        Err(err) => {
            // Fallback search based on the City/Town name
            let prefixes = vec![
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
            ];

            if let Some(tz) = prefixes.into_iter().find_map(|prefix| -> Option<Tz> {
                match format!("{prefix}/{name}").parse() {
                    Ok(tz) => Some(tz),
                    Err(_) => None,
                }
            }) {
                Ok(tz)
            } else {
                Err(err)
            }
        }
    }
}
