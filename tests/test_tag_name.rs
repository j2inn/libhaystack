// Copyright (C) 2020 - 2022, J2 Innovations

//! Test tag name utilities (to_tag_name, is_valid_tag_name)
//!
//! All test cases are ported from the TypeScript sister library
//! haystack-core/spec/core/Util.spec.ts

#[cfg(test)]
use libhaystack::util::{is_valid_tag_name, to_tag_name};

// ---------------------------------------------------------------------------
// to_tag_name
// ---------------------------------------------------------------------------

#[test]
fn test_to_tag_name_empty_string_returns_empty() {
    assert_eq!(to_tag_name(""), "empty");
}

#[test]
fn test_to_tag_name_all_illegal_chars_returns_empty() {
    assert_eq!(to_tag_name("!\"£$%^"), "empty");
}

#[test]
fn test_to_tag_name_sentence_to_camel_case() {
    assert_eq!(
        to_tag_name("oh what a time to be alive"),
        "ohWhatATimeToBeAlive"
    );
}

#[test]
fn test_to_tag_name_snake_case_unchanged() {
    assert_eq!(
        to_tag_name("oh_what_a_time_to_be_alive"),
        "oh_what_a_time_to_be_alive"
    );
}

#[test]
fn test_to_tag_name_removes_illegal_characters() {
    assert_eq!(to_tag_name("£$%test&*( this!"), "testThis");
}

#[test]
fn test_to_tag_name_replaces_dot_with_underscore() {
    assert_eq!(to_tag_name("test.me"), "test_me");
}

#[test]
fn test_to_tag_name_replaces_hyphen_with_underscore() {
    assert_eq!(to_tag_name("test-me"), "test_me");
}

#[test]
fn test_to_tag_name_replaces_slash_with_underscore() {
    assert_eq!(to_tag_name("test/me"), "test_me");
}

#[test]
fn test_to_tag_name_air_temp() {
    assert_eq!(to_tag_name("AIR TEMP"), "airTEMP");
}

#[test]
fn test_to_tag_name_air_temp_mixed_1() {
    assert_eq!(to_tag_name("AiR TEMP"), "aiRTEMP");
}

#[test]
fn test_to_tag_name_air_temp_mixed_2() {
    assert_eq!(to_tag_name("aIR TEMP"), "aIRTEMP");
}

#[test]
fn test_to_tag_name_air_temp_mixed_3() {
    assert_eq!(to_tag_name("AIrR TEMP"), "airRTEMP");
}

#[test]
fn test_to_tag_name_single_hyphen_becomes_v() {
    assert_eq!(to_tag_name("-"), "v");
}

#[test]
fn test_to_tag_name_trailing_hyphen_dropped() {
    assert_eq!(to_tag_name("v-"), "v");
}

#[test]
fn test_to_tag_name_trailing_hyphen_in_sentence_dropped() {
    assert_eq!(to_tag_name("this is a test -"), "thisIsATest");
}

#[test]
fn test_to_tag_name_first_char_uppercase_lowercased() {
    assert_eq!(to_tag_name("Hello"), "hello");
}

#[test]
fn test_to_tag_name_leading_digit_prefixed_with_v() {
    assert_eq!(to_tag_name("1test"), "v1test");
}

#[test]
fn test_to_tag_name_all_digits_prefixed_with_v() {
    assert_eq!(to_tag_name("0123456789"), "v0123456789");
}

#[test]
fn test_to_tag_name_leading_underscore_prefixed_with_v() {
    assert_eq!(to_tag_name("_foo"), "v_foo");
}

// ---------------------------------------------------------------------------
// is_valid_tag_name
// ---------------------------------------------------------------------------

#[test]
fn test_is_valid_tag_name_empty_string_returns_false() {
    assert!(!is_valid_tag_name(""));
}

#[test]
fn test_is_valid_tag_name_single_char_returns_true() {
    assert!(is_valid_tag_name("a"));
}

#[test]
fn test_is_valid_tag_name_valid_tag_returns_true() {
    assert!(is_valid_tag_name("aValidTag123"));
}

#[test]
fn test_is_valid_tag_name_spaces_returns_false() {
    assert!(!is_valid_tag_name("what a wonderful world"));
}

#[test]
fn test_is_valid_tag_name_uppercase_first_char_returns_false() {
    assert!(!is_valid_tag_name("AValidTag"));
}

#[test]
fn test_is_valid_tag_name_digit_first_char_returns_false() {
    assert!(!is_valid_tag_name("1ValidTag"));
}

#[test]
fn test_is_valid_tag_name_illegal_chars_returns_false() {
    assert!(!is_valid_tag_name("aTa£$%g"));
}
