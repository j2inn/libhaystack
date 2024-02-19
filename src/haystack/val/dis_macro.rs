// Copyright (C) 2020 - 2024, J2 Innovations

use std::{borrow::Cow, sync::OnceLock};

use super::Value;
use regex::{Captures, Regex};

/// Process a macro pattern with the given scope of name/value pairs. Also includes localization processing.
///
/// * The pattern is a unicode string with embedded expressions:
/// * '$tag': resolve tag name from scope, variable name ends with first non-tag character.
/// * '${tag}': resolve tag name from scope.
/// * '$<pod::key> localization key.
///
/// Any variables which cannot be resolved are resolved in the scope are returned as-is (i.e. $name)
/// in the result string.
///
/// If a tag resolves to a Ref, then we use the Ref.dis for the string.
pub fn dis_macro<'a, GetValueFunc, GetLocalizedFunc>(
    pattern: &str,
    get_value: GetValueFunc,
    get_localized: GetLocalizedFunc,
) -> String
where
    GetValueFunc: Fn(&str) -> Option<&'a Value>,
    GetLocalizedFunc: Fn(&str) -> Option<Cow<'a, str>>,
{
    let value_replacer = |caps: &Captures| -> Cow<str> {
        if let Some(cap_match) = caps.get(1) {
            if let Some(value) = get_value(cap_match.as_str()) {
                return match value {
                    Value::Ref(val) => Cow::Borrowed(val.dis.as_ref().unwrap_or(&val.value)),
                    Value::Str(val) => Cow::Borrowed(&val.value),
                    _ => Cow::Owned(value.to_string()),
                };
            }
        }

        // Zero is always available as per the documentation for `Regex`.
        Cow::Owned(caps.get(0).unwrap().as_str().to_string())
    };

    // Replace $tag
    static VARIABLE_REG_EX: OnceLock<Regex> = OnceLock::new();
    let variable_regex =
        VARIABLE_REG_EX.get_or_init(|| Regex::new(r"\$([a-z][a-zA-Z0-9_]+)").unwrap());

    let result = variable_regex.replace_all(pattern, value_replacer);

    // Replace ${tag}
    static SCOPE_REG_EX: OnceLock<Regex> = OnceLock::new();
    let scope_regex =
        SCOPE_REG_EX.get_or_init(|| Regex::new(r"\$\{([a-z][a-zA-Z0-9_]+)\}").unwrap());

    let result = scope_regex.replace_all(&result, value_replacer);

    let localized_replacer = |caps: &Captures| -> Cow<str> {
        if let Some(cap_match) = caps.get(1) {
            if let Some(value) = get_localized(cap_match.as_str()) {
                return value;
            }
        }

        // Zero is always available as per the documentation for `Regex`.
        Cow::Owned(caps.get(0).unwrap().as_str().to_string())
    };

    // Replace $<pod::key>
    static LOCALIZED_REG_EX: OnceLock<Regex> = OnceLock::new();
    let localized_regex = LOCALIZED_REG_EX.get_or_init(|| Regex::new(r"\$<([^>]+)>").unwrap());

    let result = localized_regex.replace_all(&result, localized_replacer);

    result.to_string()
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use std::sync::OnceLock;

    use super::dis_macro;
    use crate::dict;
    use crate::val::Dict;
    use crate::val::Value;

    static DICT: OnceLock<Dict> = OnceLock::new();

    fn dict_cb<'a>(name: &str) -> Option<&'a Value> {
        DICT.get_or_init(|| {
            dict![
                "equipRef" => Value::make_ref("ref"),
                "siteRef" => Value::make_ref_with_dis("ref2", "refDis"),
                "string" => Value::make_str("str"),
                "number" => Value::make_number(2.1),
                "boolean" => Value::make_bool(true)
            ]
        })
        .get(name)
    }

    fn i18n_cb<'a>(name: &str) -> Option<Cow<'a, str>> {
        if name == "pod::hello" {
            Some(Cow::Borrowed("world"))
        } else {
            None
        }
    }

    #[test]
    fn test_does_not_modify() {
        assert_eq!(
            dis_macro("Should not be modified!", dict_cb, i18n_cb),
            "Should not be modified!"
        );
    }

    #[test]
    fn test_does_not_modify_with_vars() {
        assert_eq!(
            dis_macro("Should not be $modified at all!", dict_cb, i18n_cb),
            "Should not be $modified at all!"
        );
    }

    #[test]
    fn test_replaces_ref_var() {
        assert_eq!(
            dis_macro("test my $equipRef bar", dict_cb, i18n_cb),
            "test my ref bar"
        );
    }

    #[test]
    fn test_replaces_ref_dis_var() {
        assert_eq!(
            dis_macro("test my $siteRef bar", dict_cb, i18n_cb),
            "test my refDis bar"
        );
    }

    #[test]
    fn test_replaces_ref_scope() {
        assert_eq!(
            dis_macro("test my ${equipRef} bar", dict_cb, i18n_cb),
            "test my ref bar"
        );
    }

    #[test]
    fn test_replaces_number() {
        assert_eq!(
            dis_macro("test my $number bar", dict_cb, i18n_cb),
            "test my 2.1 bar"
        );
    }

    #[test]
    fn test_replaces_string() {
        assert_eq!(
            dis_macro("test my $string bar", dict_cb, i18n_cb),
            "test my str bar"
        );
    }

    #[test]
    fn test_replaces_boolean() {
        assert_eq!(
            dis_macro("test my $boolean bar", dict_cb, i18n_cb),
            "test my true bar"
        );
    }

    #[test]
    fn test_replaces_localized_key() {
        assert_eq!(
            dis_macro("test my $<pod::hello> bar", dict_cb, i18n_cb),
            "test my world bar"
        );
    }

    #[test]
    fn test_replace_localized_key_not_modify() {
        assert_eq!(
            dis_macro("test my $<pod::foobie> bar", dict_cb, i18n_cb),
            "test my $<pod::foobie> bar"
        );
    }

    #[test]
    fn test_replaces_multiple_vars() {
        assert_eq!(
            dis_macro(
                "test my $equipRef $string $number $boolean $<pod::hello> bar",
                dict_cb,
                i18n_cb
            ),
            "test my ref str 2.1 true world bar"
        );
    }
}
