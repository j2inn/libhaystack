// Copyright (C) 2020 - 2022, J2 Innovations

//! Test DateTime

#[cfg(test)]
use libhaystack::val::*;

#[cfg(all(test, feature = "timezone-db"))]
use libhaystack::timezone::find_timezone;

#[test]
fn test_datetime_make_value() {
    let datetime =
        DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23-07:00", "Los_Angeles")
            .expect("DateTime");

    assert_eq!(datetime.to_rfc3339(), "2021-06-19T19:48:23-07:00");

    let value: Value = datetime.into();

    assert!(value.is_datetime());

    assert_eq!(
        DateTime::try_from(&value),
        Ok(
            DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23-07:00", "Los_Angeles")
                .unwrap()
        )
    );

    assert!(DateTime::parse_from_rfc3339("2021-06-1919:48:23-10:00").is_err());
}

#[cfg(feature = "timezone-db")]
#[test]
fn test_find_timezone() {
    // Full IANA name
    assert_eq!(
        find_timezone("America/Los_Angeles").map(|tz| tz.name()),
        Ok("America/Los_Angeles")
    );

    // Short (city) name, resolved by searching the known prefixes
    assert_eq!(
        find_timezone("Los_Angeles").map(|tz| tz.name()),
        Ok("America/Los_Angeles")
    );
    assert_eq!(
        find_timezone("Kolkata").map(|tz| tz.name()),
        Ok("Asia/Kolkata")
    );
    assert_eq!(find_timezone("UTC").map(|tz| tz.name()), Ok("UTC"));

    // Nested prefixes, e.g. America/Argentina
    assert_eq!(
        find_timezone("Ushuaia").map(|tz| tz.name()),
        Ok("America/Argentina/Ushuaia")
    );

    // Unknown names are an error
    assert!(find_timezone("Not_A_City").is_err());
    assert!(find_timezone("").is_err());
}

#[cfg(feature = "timezone-db")]
#[test]
fn test_datetime_to_fixed_offset_preserves_local_offset() {
    // PDT, -07:00
    let datetime =
        DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23-07:00", "Los_Angeles")
            .expect("DateTime");
    assert_eq!(
        datetime.to_fixed_offset().to_rfc3339(),
        "2021-06-19T19:48:23-07:00"
    );

    // PST after DST ends, -08:00
    let datetime =
        DateTime::parse_from_rfc3339_with_timezone("2021-12-19T19:48:23-08:00", "Los_Angeles")
            .expect("DateTime");
    assert_eq!(
        datetime.to_fixed_offset().to_rfc3339(),
        "2021-12-19T19:48:23-08:00"
    );

    // Half-hour offset, +05:30
    let datetime =
        DateTime::parse_from_rfc3339_with_timezone("2021-06-19T19:48:23+05:30", "Kolkata")
            .expect("DateTime");
    assert_eq!(
        datetime.to_fixed_offset().to_rfc3339(),
        "2021-06-19T19:48:23+05:30"
    );

    // UTC stays +00:00
    let datetime = DateTime::utc_now();
    assert_eq!(datetime.to_fixed_offset().offset().local_minus_utc(), 0);
}

#[test]
fn test_datetime_from_str_value() {
    let datetime = DateTime::parse_from_rfc3339("2021-06-19T19:48:23-00:00").expect("DateTime");

    assert!(datetime.is_utc());

    assert_eq!(datetime.to_string(), "2021-06-19T19:48:23UTC");
}
