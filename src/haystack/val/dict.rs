// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Dict

use crate::{dict_get, dict_has};

use crate::haystack::val::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::{FromIterator, Iterator};
use std::ops::{Deref, DerefMut};

// Alias for the underlying Dict type
pub type DictType = BTreeMap<String, Value>;

/// A Haystack Dictionary
///
/// Uses a [BTreeMap<String, Value>](std::collections::BTreeMap) for the back-store
///
/// # Example
/// Create a dictionary value
/// ```
/// use libhaystack::*;
/// use libhaystack::val::*;
///
/// let dict = Value::from(dict! {
///        "site" => Value::make_marker(),
///        "name" => Value::make_str("Foo")
///    });
/// assert!(dict.is_dict());
///
/// // Get the Dict value
/// let dict_value = Dict::try_from(&dict).unwrap();
/// assert!(!dict_value.is_empty());
/// assert!(dict_value.has("site"));
///
/// // Get a `Str` value from the dictionary
/// assert_eq!(dict_value.get_str("name"), Some(&"Foo".into()));
///```
#[derive(Eq, PartialEq, Hash, Clone, Debug, Default)]
pub struct Dict {
    value: DictType,
}

/// Dictionary trait with utilities that help working with
/// the haystack Dict types.
pub trait HaystackDict {
    /// Get the optional `id` of this `Dict`
    fn id(&self) -> Option<&Ref>;

    /// Get the `id` Ref of this `Dict`, or a default Ref if the id is not present
    fn safe_id(&self) -> Ref;

    /// Get the optional `mod` of this `Dict`.
    /// On record `Dict`s this represents the last time this
    /// record has been changed, or the time it was created.
    fn ts(&self) -> Option<&DateTime>;

    /// True if Dict contains the key
    fn has(&self, key: &str) -> bool;

    /// True if key is not found
    fn missing(&self, key: &str) -> bool;

    /// True if key exists and is a Marker
    fn has_marker(&self, key: &str) -> bool;

    /// True if key exists and is a Na
    fn has_na(&self, key: &str) -> bool;

    /// True if key exists and is a Remove
    fn has_remove(&self, key: &str) -> bool;

    /// Get optional Bool for the key
    fn get_bool<'a>(&'a self, key: &str) -> Option<&'a Bool>;

    /// Get optional Number for the key
    fn get_num<'a>(&'a self, key: &str) -> Option<&'a Number>;

    /// Get optional Ref for the key
    fn get_ref<'a>(&'a self, key: &str) -> Option<&'a Ref>;

    /// Get optional Str for the key
    fn get_str<'a>(&'a self, key: &str) -> Option<&'a Str>;

    /// Get optional XStr for the key
    fn get_xstr<'a>(&'a self, key: &str) -> Option<&'a XStr>;

    /// Get optional Uri for the key
    fn get_uri<'a>(&'a self, key: &str) -> Option<&'a Uri>;

    /// Get optional Symbol for the key
    fn get_symbol<'a>(&'a self, key: &str) -> Option<&'a Symbol>;

    /// Get optional Date for the key
    fn get_date<'a>(&'a self, key: &str) -> Option<&'a Date>;

    /// Get optional Time for the key
    fn get_time<'a>(&'a self, key: &str) -> Option<&'a Time>;

    /// Get optional DateTime for the key
    fn get_date_time<'a>(&'a self, key: &str) -> Option<&'a DateTime>;

    /// Get optional Coord for the key
    fn get_coord<'a>(&'a self, key: &str) -> Option<&'a Coord>;

    /// Get optional Dict for the key
    fn get_dict<'a>(&'a self, key: &str) -> Option<&'a Dict>;

    /// Get optional List for the key
    fn get_list<'a>(&'a self, key: &str) -> Option<&'a List>;

    /// Get optional Grid for the key
    fn get_grid<'a>(&'a self, key: &str) -> Option<&'a Grid>;

    /// Get a formatted display string for a dict.
    fn dis(&self) -> Cow<'_, str>;
}

impl Dict {
    /// Construct a new `Dict`
    pub fn new() -> Dict {
        Dict {
            value: DictType::new(),
        }
    }
}

/// Implement FromIterator for `Dict`
///
/// Allows constructing a `Dict` from a `(String, Value)` tuple iterator
impl FromIterator<(String, Value)> for Dict {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Dict {
            value: DictType::from_iter(iter),
        }
    }
}

/// Proxy method calls to the `Dict`'s `value` member
impl Deref for Dict {
    type Target = DictType;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Proxy method calls to the mutable `Dict`'s `value` member
impl DerefMut for Dict {
    #[inline]
    fn deref_mut(&mut self) -> &mut DictType {
        &mut self.value
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Dict {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Dict {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_empty() && other.is_empty() {
            std::cmp::Ordering::Equal
        } else {
            let keys_cmp = self.value.keys().cmp(other.value.keys());
            if keys_cmp == std::cmp::Ordering::Equal {
                self.value.values().cmp(other.value.values())
            } else {
                keys_cmp
            }
        }
    }
}

impl HaystackDict for Dict {
    fn id(&self) -> Option<&Ref> {
        self.get_ref("id")
    }

    fn safe_id(&self) -> Ref {
        self.get_ref("id").map_or(Ref::default(), |id| id.clone())
    }

    fn ts(&self) -> Option<&DateTime> {
        self.get_date_time("mod")
    }

    fn has(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn missing(&self, key: &str) -> bool {
        !self.has(key)
    }

    fn has_marker(&self, key: &str) -> bool {
        dict_has! {self, key, Marker}
    }

    fn has_na(&self, key: &str) -> bool {
        dict_has! {self, key, Na}
    }

    fn has_remove(&self, key: &str) -> bool {
        dict_has! {self, key, Remove}
    }

    fn get_bool<'a>(&'a self, key: &str) -> Option<&'a Bool> {
        dict_get! {self, key, Bool}
    }

    fn get_num<'a>(&'a self, key: &str) -> Option<&'a Number> {
        dict_get! {self, key, Number}
    }

    fn get_str<'a>(&'a self, key: &str) -> Option<&'a Str> {
        dict_get! {self, key, Str}
    }

    fn get_xstr<'a>(&'a self, key: &str) -> Option<&'a XStr> {
        dict_get! {self, key, XStr}
    }

    fn get_ref<'a>(&'a self, key: &str) -> Option<&'a Ref> {
        dict_get! {self, key, Ref}
    }

    fn get_uri<'a>(&'a self, key: &str) -> Option<&'a Uri> {
        dict_get! {self, key, Uri}
    }

    fn get_symbol<'a>(&'a self, key: &str) -> Option<&'a Symbol> {
        dict_get! {self, key, Symbol}
    }

    fn get_date<'a>(&'a self, key: &str) -> Option<&'a Date> {
        dict_get! {self, key, Date}
    }

    fn get_time<'a>(&'a self, key: &str) -> Option<&'a Time> {
        dict_get! {self, key, Time}
    }

    fn get_date_time<'a>(&'a self, key: &str) -> Option<&'a DateTime> {
        dict_get! {self, key, DateTime}
    }

    fn get_coord<'a>(&'a self, key: &str) -> Option<&'a Coord> {
        dict_get! {self, key, Coord}
    }

    fn get_dict<'a>(&'a self, key: &str) -> Option<&'a Dict> {
        dict_get! {self, key, Dict}
    }

    fn get_list<'a>(&'a self, key: &str) -> Option<&'a List> {
        dict_get! {self, key, List}
    }

    fn get_grid<'a>(&'a self, key: &str) -> Option<&'a Grid> {
        dict_get! {self, key, Grid}
    }

    fn dis(&self) -> Cow<'_, str> {
        dict_to_dis(self, &|_| None, None)
    }
}

/// Converts from `DictType` to a `Dict`
impl From<DictType> for Dict {
    fn from(from: DictType) -> Self {
        Dict { value: from }
    }
}

/// Converts from `DictType` to a `Dict` `Value`
impl From<DictType> for Value {
    fn from(from: DictType) -> Self {
        Value::from(Dict { value: from })
    }
}

/// Converts from `Dict` to a `Dict` `Value`
impl From<Dict> for Value {
    fn from(value: Dict) -> Self {
        Value::Dict(value)
    }
}

/// Tries to convert from `Value` to a `Dict`
impl TryFrom<&Value> for Dict {
    type Error = &'static str;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dict(v) => Ok(v.clone()),
            _ => Err("Value is not an `Dict`"),
        }
    }
}

/// Pretty print this
impl Display for Dict {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

/// A macro for creating a [Dict](crate::val::Dict) from literals
///
/// # Example
/// ```
///  use libhaystack::*;
///  use libhaystack::val::*;
///     let dict = dict!{
///         "site" => Value::make_marker(),
///         "dis" => Value::make_str("Some site")
///     };
/// ```
///
#[macro_export]
macro_rules! dict(
    { $($key:expr => $value:expr),* $(,)? } => {
        {
            let mut map = ::std::collections::BTreeMap::new();
            $(
                map.insert(String::from($key), $value);
            )+
            Dict::from(map)
        }
     };
);

/// A macro for retrieving a type from a [Dict](crate::val::Dict) by a key
///
/// This is a private API, consider using the [Dict](crate::val::Dict) specialized functions for
/// getting the values.
///
#[macro_export]
macro_rules! dict_get(
    { $self:ident, $key:expr, $type:ident } => {
        {
            if let Some(value) = $self.get($key) {
                match value {
                    Value::$type(val) => Some(&val),
                    _ => None,
                }
            } else {
                None
            }
    }
     };
);

/// A macro for determining if [Dict](crate::val::Dict) has a type for the key
///
/// Private API, use the [Dict](crate::val::Dict) specialized functions
///
#[macro_export]
macro_rules! dict_has(
    { $self:ident, $key:expr, $type:ident } => {
        {
            let entry = $self.get($key);
            matches!(entry, Some(Value::$type))
        }
     };
);

/// Convert a dict to its formatted display string.
pub fn dict_to_dis<'a, GetLocalizedFunc>(
    dict: &'a Dict,
    get_localized: &'a GetLocalizedFunc,
    def: Option<Cow<'a, str>>,
) -> Cow<'a, str>
where
    GetLocalizedFunc: Fn(&str) -> Option<Cow<'a, str>>,
{
    if let Some(val) = dict.get("dis") {
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("disMacro") {
        return match val {
            Value::Str(val) => dis_macro(&val.value, |val| dict.get(val), get_localized),
            _ => decode_str_from_value(val),
        };
    }

    if let Some(val) = dict.get("disKey") {
        if let Value::Str(val) = val {
            if let Some(val) = get_localized(&val.value) {
                return val;
            }
        }
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("name") {
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("def") {
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("tag") {
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("id") {
        return match val {
            Value::Ref(val) => Cow::Borrowed(val.dis.as_ref().unwrap_or(&val.value)),
            _ => decode_str_from_value(val),
        };
    }

    def.unwrap_or(Cow::Borrowed(""))
}

fn decode_str_from_value(val: &'_ Value) -> Cow<'_, str> {
    match val {
        Value::Str(val) => Cow::Borrowed(&val.value),
        _ => Cow::Owned(val.to_string()),
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::val::{dict_to_dis, Dict, HaystackDict, Value};

    fn get_localized<'a>(key: &str) -> Option<Cow<'a, str>> {
        match key {
            "key" => Some(Cow::Borrowed("translated")),
            _ => None,
        }
    }

    #[test]
    fn dict_to_dis_returns_dis() {
        let dict = dict!["dis" => Value::make_str("display")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "display");
    }

    #[test]
    fn dict_to_dis_returns_dis_not_str() {
        let dict = dict!["dis" => Value::make_ref("display")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "@display");
    }

    #[test]
    fn dict_to_dis_returns_dis_macro() {
        let dict = dict!["foo" => Value::make_str("bar"), "disMacro" => Value::make_str("hello $foo world!")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "hello bar world!");
    }

    #[test]
    fn dict_to_dis_returns_dis_key_translated() {
        let dict = dict!["foo" => Value::make_str("bar"), "disKey" => Value::make_str("key")];
        assert_eq!(dict_to_dis(&dict, &get_localized, None), "translated");
    }

    #[test]
    fn dict_to_dis_returns_dis_key_not_translated() {
        let dict =
            dict!["foo" => Value::make_str("bar"), "disKey" => Value::make_str("notTranslated")];
        assert_eq!(dict_to_dis(&dict, &get_localized, None), "notTranslated");
    }

    #[test]
    fn dict_to_dis_returns_name() {
        let dict = dict!["name" => Value::make_str("display")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "display");
    }

    #[test]
    fn dict_to_dis_returns_def() {
        let dict = dict!["def" => Value::make_str("display")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "display");
    }

    #[test]
    fn dict_to_dis_returns_tag() {
        let dict = dict!["tag" => Value::make_str("display")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "display");
    }

    #[test]
    fn dict_to_dis_returns_id() {
        let dict = dict!["id" => Value::make_ref("id")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "id");
    }

    #[test]
    fn dict_to_dis_returns_id_dis() {
        let dict = dict!["id" => Value::make_ref_with_dis("id", "dis")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "dis");
    }

    #[test]
    fn dict_returns_dis() {
        let dict = dict!["dis" => Value::make_str("display")];
        assert_eq!(dict.dis(), "display");
    }

    #[test]
    fn dict_returns_default_value_if_none_found() {
        let dict = dict!["something" => Value::make_str("display")];
        assert_eq!(
            dict_to_dis(&dict, &|_| None, Some("default".into())),
            "default"
        );
    }
}
