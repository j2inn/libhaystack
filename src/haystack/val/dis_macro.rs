// Copyright (C) 2020 - 2024, J2 Innovations

use std::{borrow::Cow, sync::OnceLock};

use super::Value;
use regex::{Captures, Regex, Replacer};

/// A regular expression replacer implementation for replacing formatted
/// values in a display macro string.
struct DisReplacer<'a, 'b, GetValueFunc, GetLocalizedFunc>
where
    GetValueFunc: Fn(&str) -> Option<Cow<'a, Value>>,
    GetLocalizedFunc: Fn(&str) -> Option<Cow<'a, str>>,
{
    get_value: &'b GetValueFunc,

    get_localized: &'b GetLocalizedFunc,
}

impl<'a, 'b, GetValueFunc, GetLocalizedFunc> Replacer
    for DisReplacer<'a, 'b, GetValueFunc, GetLocalizedFunc>
where
    GetValueFunc: Fn(&str) -> Option<Cow<'a, Value>>,
    GetLocalizedFunc: Fn(&str) -> Option<Cow<'a, str>>,
{
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let mut default_replace = || dst.push_str(caps.get(0).unwrap().as_str());

        // Replace $tag or ${tag}
        if let Some(cap_match) = caps.get(2).or_else(|| caps.get(4)) {
            if let Some(value) = (self.get_value)(cap_match.as_str()) {
                if let Value::Ref(val) = value.as_ref() {
                    dst.push_str(val.dis.as_ref().unwrap_or(&val.value));
                } else if let Value::Str(val) = value.as_ref() {
                    dst.push_str(&val.value);
                } else {
                    dst.push_str(&value.to_string());
                }
            } else {
                default_replace();
            }
        }
        // Replace $<pod::key>
        else if let Some(cap_match) = caps.get(6) {
            if let Some(value) = (self.get_localized)(cap_match.as_str()) {
                dst.push_str(&value);
            } else {
                default_replace();
            }
        } else {
            default_replace();
        }
    }
}

/// Process a macro pattern with the given scope of name/value pairs. Also includes localization processing.
///
/// The pattern is a unicode string with embedded expressions:
/// * `$tag`: resolve tag name from scope, variable name ends with first non-tag character.
/// * `${tag}`: resolve tag name from scope.
/// * `$<pod::key>` localization key.
///
/// Any variables which cannot be resolved are resolved in the scope are returned as-is (i.e. `$name`)
/// in the result string.
///
/// If a tag resolves to a `Ref`, then we use the `Ref.dis` for the string.
pub fn dis_macro<'a, GetValueFunc, GetLocalizedFunc>(
    pattern: &'a str,
    get_value: GetValueFunc,
    get_localized: GetLocalizedFunc,
) -> Cow<'a, str>
where
    GetValueFunc: Fn(&str) -> Option<Cow<'a, Value>>,
    GetLocalizedFunc: Fn(&str) -> Option<Cow<'a, str>>,
{
    // Cache the regular expression in memory so it doesn't need to compile on each invocation.
    static REG_EX: OnceLock<Regex> = OnceLock::new();

    REG_EX
        .get_or_init(|| {
            // Replace $tags, ${tag} or $<pod::key>
            Regex::new(r"(\$([a-z][a-zA-Z0-9_]+))|(\$\{([a-z][a-zA-Z0-9_]+)\})|(\$<([^>]+)>)")
                .unwrap()
        })
        .replace_all(
            pattern,
            DisReplacer {
                get_value: &get_value,
                get_localized: &get_localized,
            },
        )
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

    fn dict_cb<'a>(name: &str) -> Option<Cow<'a, Value>> {
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
        .map(|val| Cow::Borrowed(val))
    }

    fn i18n_cb<'a>(name: &str) -> Option<Cow<'a, str>> {
        match name {
            "pod::hello" => Some(Cow::Borrowed("world")),
            _ => None,
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
