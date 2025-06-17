// Copyright (C) 2020 - 2022, J2 Innovations

//! Zinc Date, Time, and DateTime scalar decoding

use super::super::scanner::Scanner;
use crate::haystack::timezone::make_date_time_with_tz;
use crate::haystack::val::date::Date;
use crate::haystack::val::datetime::DateTime;
use crate::haystack::val::time::Time;
use chrono::Datelike;
use chrono::Timelike;
use chrono::{Duration, FixedOffset, Offset, TimeZone, Utc};

use std::io::{Error, Read};
use std::ops::Deref;
use std::str::FromStr;

const DIGITS: std::ops::RangeInclusive<u8> = b'0'..=b'9';

/// Parse Time
pub(crate) fn parse_time<R: Read>(scanner: &mut Scanner<R>) -> Result<Time, Error> {
    let mut time_str: Vec<u8> = vec![
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume(b':')?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume(b':')?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
    ];

    if scanner.cur == b'.' {
        time_str.push(scanner.cur);
        scanner.read()?;
        while !scanner.is_eof && scanner.is_digit() {
            time_str.push(scanner.cur);
            scanner.advance()?
        }
    }

    let time_str = &String::from_utf8_lossy(&time_str);
    match Time::from_str(time_str) {
        Ok(time) => Ok(time),
        Err(err) => scanner.make_generic_err(&format!(
            "Invalid time format '{time_str}'. Parse error: {err}"
        )),
    }
}

/// Parse Date
pub(crate) fn parse_date<R: Read>(scanner: &mut Scanner<R>) -> Result<Date, Error> {
    let date: Vec<u8> = vec![
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume(b'-')?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume(b'-')?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
        scanner.expect_and_consume_any_in_range(&DIGITS)?,
    ];

    as_date(&date, scanner)
}

pub(crate) fn as_date<R: Read>(buf: &[u8], scanner: &mut Scanner<R>) -> Result<Date, Error> {
    let date_str = &String::from_utf8_lossy(buf);
    match Date::from_str(date_str) {
        Ok(date) => Ok(date),
        Err(err) => scanner.make_generic_err(&format!(
            "Invalid date format '{date_str}'. Parse error: {err}"
        )),
    }
}

pub(crate) fn is_partial_date<R: Read>(scanner: &mut Scanner<R>) -> Result<bool, Error> {
    Ok(DIGITS.contains(&scanner.peek()?)
        && DIGITS.contains(&scanner.peek()?)
        && scanner.peek()? == (b'-')
        && DIGITS.contains(&scanner.peek()?)
        && DIGITS.contains(&scanner.peek()?))
}

/// Parse DateTime
pub(crate) fn parse_datetime<R: Read>(scanner: &mut Scanner<R>) -> Result<DateTime, Error> {
    let date = parse_date(scanner)?;

    scanner.expect_and_consume(b'T')?;

    let time = parse_time(scanner)?;

    let (tz, fixed_offset) = parse_time_zone(scanner)?;

    let datetime = date.and_time(*time.deref());
    let utc = Utc.from_utc_datetime(&datetime);
    if tz == "UTC" {
        Ok(utc.into())
    } else {
        fixed_offset
            .map_or_else(
                || make_date_time_with_tz(&utc.with_timezone(&Utc.fix()), &tz),
                |offset| {
                    offset
                        .with_ymd_and_hms(
                            date.year(),
                            date.month(),
                            date.day(),
                            time.hour(),
                            time.minute(),
                            time.second(),
                        )
                        .single()
                        .and_then(|dt| dt.with_nanosecond(time.nanosecond()))
                        .ok_or_else(|| String::from("Invalid date time."))
                        .and_then(|fixed| make_date_time_with_tz(&fixed, &tz))
                },
            )
            .map(DateTime::from)
            .or_else(|err| scanner.make_generic_err(&err))
    }
}

fn parse_time_zone<R: Read>(
    scanner: &mut Scanner<R>,
) -> Result<(String, Option<FixedOffset>), Error> {
    if scanner.cur == b'Z' {
        if scanner.safe_peek() == Some(b' ')
            && if let Some(char) = scanner.safe_peek() {
                char.is_ascii_uppercase()
            } else {
                false
            }
        {
            scanner.advance_by(2)?;
            Ok((parse_time_zone_name(scanner)?, None))
        } else {
            if !scanner.is_eof {
                scanner.read()?;
            }
            Ok(("UTC".into(), None))
        }
    } else {
        let offset = vec![
            scanner.expect_and_consume_any_of("+-")?,
            scanner.expect_and_consume_any_in_range(&DIGITS)?,
            scanner.expect_and_consume_any_in_range(&DIGITS)?,
            scanner.expect_and_consume(b':')?,
            scanner.expect_and_consume_any_in_range(&DIGITS)?,
            scanner.expect_and_consume_any_in_range(&DIGITS)?,
        ];

        scanner.expect_and_consume(b' ')?;

        let gmt_offset = &String::from_utf8_lossy(&offset);

        let gmt_sign = gmt_offset[0..1].to_string();

        let dur = Duration::hours(gmt_offset[1..3].parse::<i64>().unwrap_or(0))
            + Duration::minutes(gmt_offset[4..6].parse::<i64>().unwrap_or(0));

        let tz_name = parse_time_zone_name(scanner)?;

        let fixed_offset = if gmt_sign == "+" {
            FixedOffset::east_opt(dur.num_seconds() as i32)
        } else {
            FixedOffset::west_opt(dur.num_seconds() as i32)
        };

        Ok((tz_name, fixed_offset))
    }
}

fn parse_time_zone_name<R: Read>(scanner: &mut Scanner<R>) -> Result<String, Error> {
    let mut name = vec![scanner.expect_and_consume_any_in_range(&(b'A'..=b'Z'))?];

    while !scanner.is_eof && (scanner.is_alpha_num() || scanner.is_any_of("_/+-")) {
        name.push(scanner.cur);
        scanner.advance()?
    }

    let name = String::from_utf8_lossy(&name).to_string();
    if name.len() == 1 {
        scanner.make_generic_err(&format!("Invalid timezone name '{name}'."))
    } else {
        Ok(name)
    }
}

#[cfg(test)]
mod test {
    use super::{parse_date, parse_datetime, parse_time};
    use crate::haystack::val::{Date, DateTime, Time};
    use std::io::Cursor;
    use std::str::FromStr;

    #[test]
    fn test_zinc_parse_time() {
        let mut input = Cursor::new("12:23:45.1993".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let time = parse_time(&mut scanner);
        assert_eq!(
            time.ok(),
            Some(Time::from_str("12:23:45.1993").expect("Time"))
        )
    }

    #[test]
    fn test_zinc_parse_date() {
        let mut input = Cursor::new("2020-08-31".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let date = parse_date(&mut scanner);
        assert_eq!(date.ok(), Some(Date::from_str("2020-08-31").expect("Date")))
    }

    #[test]
    fn test_zinc_parse_datetime() {
        let mut input = Cursor::new("2020-08-31T12:46:30.234Z UTC".as_bytes());
        let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

        let datetime = parse_datetime(&mut scanner);
        assert_eq!(
            datetime.ok(),
            Some(DateTime::from_str("2020-08-31T12:46:30.234Z").expect("DateTime"))
        );

        {
            let mut input = Cursor::new("2020-08-31T12:46:30.234Z ".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let datetime = parse_datetime(&mut scanner);
            assert_eq!(
                datetime.ok(),
                Some(DateTime::from_str("2020-08-31T12:46:30.234Z").expect("DateTime"))
            );
        }

        {
            let mut input = Cursor::new("2010-03-11T23:55:00-05:00 New_York".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let datetime = parse_datetime(&mut scanner);
            assert_eq!(
                datetime.ok(),
                Some(DateTime::from_str("2010-03-11T23:55:00-05:00").expect("DateTime"))
            )
        }

        {
            let mut input = Cursor::new("2010-11-28T07:23:02.773-04:00 Port-au-Prince".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let datetime = parse_datetime(&mut scanner);

            assert_eq!(
                datetime.ok(),
                Some(DateTime::from_str("2010-11-28T07:23:02.773-04:00").expect("DateTime"))
            )
        }

        {
            let mut input = Cursor::new("2025-06-12T15:22:11.518+02:00 GMT-2".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let datetime = parse_datetime(&mut scanner);

            assert_eq!(
                datetime.ok(),
                Some(DateTime::from_str("2025-06-12T15:22:11.518+02:00").expect("DateTime"))
            )
        }

        {
            let mut input = Cursor::new("2025-06-12T11:23:32.488-02:00 GMT+2".as_bytes());
            let mut scanner = super::Scanner::make(&mut input).expect("Scanner");

            let datetime = parse_datetime(&mut scanner);

            assert_eq!(
                datetime.ok(),
                Some(DateTime::from_str("2025-06-12T11:23:32.488-02:00").expect("DateTime"))
            )
        }
    }
}
