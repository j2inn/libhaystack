// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack tag name utilities.
//!
//! Provides functions for validating and converting arbitrary strings into
//! valid Haystack tag names.
//!
//! A valid tag name must match the grammar:
//! ```text
//! <alphaLo> (<alphaLo> | <alphaHi> | <digit> | '_')*
//! ```
//!
//! For more information see <https://project-haystack.org/doc/docHaystack/Kinds>

use std::borrow::Cow;

/// Returns `true` if `name` is already a valid Haystack tag name.
///
/// A valid tag name starts with a lowercase ASCII letter followed by zero or
/// more ASCII letters, digits, or underscores.
///
/// # Examples
/// ```
/// use libhaystack::util::is_valid_tag_name;
///
/// assert!(is_valid_tag_name("aValidTag123"));
/// assert!(!is_valid_tag_name("AValidTag"));
/// assert!(!is_valid_tag_name("1invalid"));
/// assert!(!is_valid_tag_name(""));
/// ```
pub fn is_valid_tag_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

/// Converts an arbitrary string into a valid Haystack tag name.
///
/// The conversion follows the same rules as the TypeScript `toTagName` utility:
///
/// 1. If the string is already a valid tag name, it is returned unchanged.
/// 2. `.`, `-`, and `/` are replaced with `_`; if at position 0 they become
///    `v`; if at the last position they are dropped.
/// 3. A leading digit or `_` is prefixed with `v`.
/// 4. All remaining invalid characters are stripped.
/// 5. Spaces trigger camelCase conversion: each word after the first has its
///    first letter uppercased (provided it is a lowercase letter).
/// 6. A leading run of uppercase letters on the first word is lowercased.
/// 7. Returns `"empty"` if no valid characters remain.
///
/// # Examples
/// ```
/// use libhaystack::util::to_tag_name;
///
/// assert_eq!(to_tag_name("oh what a time to be alive"), "ohWhatATimeToBeAlive");
/// assert_eq!(to_tag_name("AIR TEMP"), "airTEMP");
/// assert_eq!(to_tag_name("1test"), "v1test");
/// assert_eq!(to_tag_name(""), "empty");
/// assert_eq!(to_tag_name("alreadyValid"), "alreadyValid");
/// ```
pub fn to_tag_name(name: &str) -> Cow<'_, str> {
    if is_valid_tag_name(name) {
        return Cow::Borrowed(name);
    }

    // Step 1: Replace `.`, `-`, `/` with `_`, `v` (at pos 0), or drop (at last pos).
    let char_count = name.chars().count();
    let last_idx = char_count.saturating_sub(1);

    let mut step1 = String::with_capacity(name.len());
    for (i, c) in name.chars().enumerate() {
        match c {
            '.' | '-' | '/' => {
                if i == 0 {
                    step1.push('v');
                } else if i != last_idx {
                    step1.push('_');
                }
                // last position: drop the character
            }
            _ => step1.push(c),
        }
    }

    // Step 2: Prefix with `v` when the string starts with a digit or `_`.
    let step2 = if step1.starts_with(|c: char| c.is_ascii_digit() || c == '_') {
        let mut s = String::with_capacity(step1.len() + 1);
        s.push('v');
        s.push_str(&step1);
        s
    } else {
        step1
    };

    // Step 3: Remove all characters that are not ASCII alphanumeric, `_`, or space; trim.
    let step3 = step2
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
        .collect::<String>();
    let step3 = step3.trim();

    // Step 4: Split on spaces, camelCase conversion.
    let result: String = step3
        .split(' ')
        .filter(|part| !part.is_empty())
        .enumerate()
        .map(|(i, part)| {
            let mut chars = part.chars();
            let start = chars.next().expect("part is non-empty after filter");

            if i == 0 {
                // Lowercase the leading run of uppercase letters on the first word.
                if start.is_ascii_uppercase() {
                    let mut new_part = String::with_capacity(part.len());
                    let mut caps_prefix = true;
                    for ch in part.chars() {
                        if caps_prefix && ch.is_ascii_uppercase() {
                            new_part.push(ch.to_ascii_lowercase());
                        } else {
                            caps_prefix = false;
                            new_part.push(ch);
                        }
                    }
                    new_part
                } else {
                    part.to_string()
                }
            } else {
                // Capitalize the first letter of subsequent words only when it is lowercase.
                if start.is_ascii_alphabetic() && start.is_ascii_lowercase() {
                    let mut new_part = String::with_capacity(part.len());
                    new_part.push(start.to_ascii_uppercase());
                    new_part.push_str(&part[1..]);
                    new_part
                } else {
                    part.to_string()
                }
            }
        })
        .collect();

    if result.is_empty() {
        Cow::Borrowed("empty")
    } else {
        Cow::Owned(result)
    }
}
