// Copyright (C) 2020 - 2022, J2 Innovations

//! Haystack Dict

use crate::{dict_get, dict_has};

use crate::haystack::val::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, Iterator};
use std::ops::Index;

// Alias for the underlying Dict type
pub(crate) type DictType = BTreeMap<String, Value>;

const SMALL_DICT_MAX_ENTRIES: usize = 32;

#[derive(Clone, Debug)]
enum DictRepr {
    Small(Vec<(String, Value)>),
    Tree(DictType),
}

/// A Haystack Dictionary
///
/// Uses a hybrid back-store: a sorted small-vector for tiny dicts and a
/// [BTreeMap<String, Value>](std::collections::BTreeMap) once the dict grows.
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
#[derive(Clone, Debug)]
pub struct Dict {
    value: DictRepr,
    small_max_entries: usize,
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
    /// Construct a new `Dict` with a threshold of 32 entries for the small-vector back-store.
    pub fn new() -> Dict {
        Self::with_small_max_entries(SMALL_DICT_MAX_ENTRIES)
    }

    /// Construct a new `Dict` with a custom small-store threshold.
    pub fn with_small_max_entries(small_max_entries: usize) -> Dict {
        let value = if small_max_entries == 0 {
            DictRepr::Tree(DictType::new())
        } else {
            DictRepr::Small(Vec::new())
        };
        Dict {
            value,
            small_max_entries,
        }
    }

    /// Return the active small-store threshold for this dict.
    pub fn small_max_entries(&self) -> usize {
        self.small_max_entries
    }

    fn small_search(entries: &[(String, Value)], key: &str) -> Result<usize, usize> {
        entries.binary_search_by(|(k, _)| k.as_str().cmp(key))
    }

    fn spill_to_tree(&mut self) {
        if let DictRepr::Small(entries) = &mut self.value {
            let map = entries.drain(..).collect::<DictType>();
            self.value = DictRepr::Tree(map);
        }
    }

    pub fn len(&self) -> usize {
        match &self.value {
            DictRepr::Small(entries) => entries.len(),
            DictRepr::Tree(map) => map.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        match &mut self.value {
            DictRepr::Small(entries) => entries.clear(),
            DictRepr::Tree(map) => map.clear(),
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        match &self.value {
            DictRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| &entries[pos].1),
            DictRepr::Tree(map) => map.get(key),
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        match &mut self.value {
            DictRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| &mut entries[pos].1),
            DictRepr::Tree(map) => map.get_mut(key),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        match &mut self.value {
            DictRepr::Small(entries) => {
                if entries.len() < self.small_max_entries
                    && entries
                        .last()
                        .is_none_or(|(last_key, _)| key.as_str() > last_key.as_str())
                {
                    entries.push((key, value));
                    return None;
                }

                match Self::small_search(entries, &key) {
                    Ok(pos) => Some(std::mem::replace(&mut entries[pos].1, value)),
                    Err(pos) => {
                        if entries.len() < self.small_max_entries {
                            entries.insert(pos, (key, value));
                            None
                        } else {
                            self.spill_to_tree();
                            match &mut self.value {
                                DictRepr::Tree(map) => map.insert(key, value),
                                DictRepr::Small(_) => None,
                            }
                        }
                    }
                }
            }
            DictRepr::Tree(map) => map.insert(key, value),
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        match &mut self.value {
            DictRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| entries.remove(pos).1),
            DictRepr::Tree(map) => map.remove(key),
        }
    }

    pub fn pop_first(&mut self) -> Option<(String, Value)> {
        match &mut self.value {
            DictRepr::Small(entries) => {
                if entries.is_empty() {
                    None
                } else {
                    Some(entries.remove(0))
                }
            }
            DictRepr::Tree(map) => map.pop_first(),
        }
    }

    /// Demote a `Tree`-backed dict back to `Small` when its entry count has
    /// dropped to at or below the small-store threshold.
    ///
    /// This is the inverse of the automatic spill that happens in [`insert`](Self::insert).
    /// Call it after a burst of [`remove`](Self::remove) calls to recover the
    /// performance and memory advantages of the sorted-vector representation.
    ///
    /// If the dict is already `Small`-backed this is a no-op.
    pub fn shrink_to_fit(&mut self) {
        if let DictRepr::Tree(map) = &self.value
            && map.len() <= self.small_max_entries
        {
            let entries = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            self.value = DictRepr::Small(entries);
        }
    }

    pub fn iter(&self) -> DictIter<'_> {
        match &self.value {
            DictRepr::Small(entries) => DictIter::Small(entries.iter()),
            DictRepr::Tree(map) => DictIter::Tree(map.iter()),
        }
    }

    pub fn iter_mut(&mut self) -> DictIterMut<'_> {
        match &mut self.value {
            DictRepr::Small(entries) => DictIterMut::Small(entries.iter_mut()),
            DictRepr::Tree(map) => DictIterMut::Tree(map.iter_mut()),
        }
    }

    pub fn keys(&self) -> DictKeys<'_> {
        DictKeys { inner: self.iter() }
    }

    pub fn values(&self) -> DictValues<'_> {
        DictValues { inner: self.iter() }
    }
}

impl Default for Dict {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl Eq for Dict {}

impl Hash for Dict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

pub enum DictIter<'a> {
    Small(std::slice::Iter<'a, (String, Value)>),
    Tree(std::collections::btree_map::Iter<'a, String, Value>),
}

impl<'a> Iterator for DictIter<'a> {
    type Item = (&'a String, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DictIter::Small(iter) => iter.next().map(|(k, v)| (k, v)),
            DictIter::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DictIter::Small(iter) => iter.size_hint(),
            DictIter::Tree(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for DictIter<'_> {}

pub enum DictIterMut<'a> {
    Small(std::slice::IterMut<'a, (String, Value)>),
    Tree(std::collections::btree_map::IterMut<'a, String, Value>),
}

impl<'a> Iterator for DictIterMut<'a> {
    type Item = (&'a String, &'a mut Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DictIterMut::Small(iter) => iter.next().map(|(k, v)| (&*k, v)),
            DictIterMut::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DictIterMut::Small(iter) => iter.size_hint(),
            DictIterMut::Tree(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for DictIterMut<'_> {}

pub enum DictIntoIter {
    Small(std::vec::IntoIter<(String, Value)>),
    Tree(std::collections::btree_map::IntoIter<String, Value>),
}

impl Iterator for DictIntoIter {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DictIntoIter::Small(iter) => iter.next(),
            DictIntoIter::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            DictIntoIter::Small(iter) => iter.size_hint(),
            DictIntoIter::Tree(iter) => iter.size_hint(),
        }
    }
}

impl ExactSizeIterator for DictIntoIter {}

impl IntoIterator for Dict {
    type Item = (String, Value);
    type IntoIter = DictIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self.value {
            DictRepr::Small(entries) => DictIntoIter::Small(entries.into_iter()),
            DictRepr::Tree(map) => DictIntoIter::Tree(map.into_iter()),
        }
    }
}

impl<'a> IntoIterator for &'a Dict {
    type Item = (&'a String, &'a Value);
    type IntoIter = DictIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Dict {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = DictIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Index<&str> for Dict {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("no entry found for key: {index}"))
    }
}

pub struct DictKeys<'a> {
    inner: DictIter<'a>,
}

impl<'a> Iterator for DictKeys<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for DictKeys<'_> {}

pub struct DictValues<'a> {
    inner: DictIter<'a>,
}

impl<'a> Iterator for DictValues<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for DictValues<'_> {}

/// Implement FromIterator for `Dict`
///
/// Allows constructing a `Dict` from a `(String, Value)` tuple iterator
impl FromIterator<(String, Value)> for Dict {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let mut dict = Dict::new();
        if lower > dict.small_max_entries
            || upper.is_some_and(|upper| upper > dict.small_max_entries)
        {
            dict.value = DictRepr::Tree(iter.collect());
            return dict;
        }

        // Reserve capacity up-front when the hint is available and fits in Small,
        // avoiding repeated Vec reallocations for the common fixed-size-collection case.
        if lower > 0
            && let DictRepr::Small(entries) = &mut dict.value
        {
            entries.reserve(lower.min(dict.small_max_entries));
        }

        for (k, v) in iter.by_ref() {
            dict.insert(k, v);
        }
        dict
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Dict {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dict {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other.iter())
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
        let small_max_entries = SMALL_DICT_MAX_ENTRIES;
        if from.len() <= small_max_entries {
            Dict {
                value: DictRepr::Small(from.into_iter().collect()),
                small_max_entries,
            }
        } else {
            Dict {
                value: DictRepr::Tree(from),
                small_max_entries,
            }
        }
    }
}

/// Converts from `DictType` to a `Dict` `Value`
impl From<DictType> for Value {
    fn from(from: DictType) -> Self {
        Value::from(Dict::from(from))
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
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Helper macro for converting dict keys.
/// Accepts either bare identifiers (`id`) or string/expression keys (`"id"`, `(KEY_ID)`).
///
/// This is exported so that `dict!` can reference it as `$crate::dict_key!`
/// when expanded at call sites outside this module.
#[doc(hidden)]
#[macro_export]
macro_rules! dict_key(
    { $key:ident } => { stringify!($key).to_string() };
    { $key:expr }  => { String::from($key) };
);

/// A macro for creating a [Dict](crate::val::Dict) from literals
///
/// Keys can be bare identifiers or string/expression literals.
///
/// Bare identifiers are stringified, so `site => ...` becomes the key `"site"`.
/// If you want to use a const or other expression as the key, wrap it in
/// parentheses, such as `(KEY_SITE) => ...`.
/// Values can be any type that implements `Into<Value>` (e.g. `bool`, `f64`,
/// `i32`, `&str`, haystack types, or explicit `Value::...` expressions).
///
/// # Example
/// ```
///  use libhaystack::*;
///  use libhaystack::val::*;
///
///     // String keys with explicit Value expressions (original syntax)
///     let dict = dict!{
///         "site" => Value::make_marker(),
///         "dis" => Value::make_str("Some site")
///     };
///
///     // Identifier keys with native/std types
///     let dict2 = dict!{
///         site => Marker,
///         dis => "Some site"
///     };
///
///     // Const/expression keys must be parenthesized
///     const KEY_SITE: &str = "site";
///     let dict3 = dict!{
///         (KEY_SITE) => Marker,
///     };
/// ```
///
#[macro_export]
macro_rules! dict(
    { $($key:tt => $value:expr),* $(,)? } => {
        {
            let mut map = Dict::new();
            $(
                map.insert($crate::dict_key!($key), Value::from($value));
            )+
            map
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
        return if let Value::Str(val) = val {
            dis_macro(
                &val.value,
                |val| dict.get(val).map(Cow::Borrowed),
                get_localized,
            )
        } else {
            decode_str_from_value(val)
        };
    }

    if let Some(val) = dict.get("disKey") {
        if let Value::Str(val_str) = val
            && let Some(val_str) = get_localized(&val_str.value)
        {
            return val_str;
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

    if let Some(val) = dict.get("navName") {
        return decode_str_from_value(val);
    }

    if let Some(val) = dict.get("id") {
        return if let Value::Ref(val) = val {
            Cow::Borrowed(val.dis.as_ref().unwrap_or(&val.value))
        } else {
            decode_str_from_value(val)
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

    use crate::val::dict::DictRepr;
    use crate::val::{Dict, HaystackDict, Value, dict_to_dis};

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
    fn dict_to_dis_returns_nav_name() {
        let dict = dict!["navName" => Value::make_str("navName")];
        assert_eq!(dict_to_dis(&dict, &|_| None, None), "navName");
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

    #[test]
    fn dict_ident_keys_and_native_values() {
        use crate::val::Marker;

        // Identifier keys, native/std value types
        let dict = dict! {
            site => Marker,
            dis  => "Some site",
            curVal => 42.0f64,
            occupied => true,
        };

        assert!(matches!(dict.get("site"), Some(Value::Marker)));
        assert_eq!(dict.get_str("dis"), Some(&"Some site".into()));
        assert!(matches!(dict.get("curVal"), Some(Value::Number(_))));
        assert!(matches!(dict.get("occupied"), Some(Value::Bool(_))));

        // Original string-key / explicit-Value syntax still works
        let dict2 = dict! {
            "site" => Value::make_marker(),
            "dis"  => Value::make_str("bar"),
        };
        assert!(dict2.has_marker("site"));
        assert_eq!(dict2.get_str("dis"), Some(&"bar".into()));
    }

    #[test]
    fn dict_parenthesized_const_key() {
        use crate::val::Marker;

        const KEY_SITE: &str = "site";
        let dict = dict! { (KEY_SITE) => Marker };

        assert!(dict.has_marker("site"));
        assert!(dict.has(KEY_SITE));
    }

    // -- Hybrid-store unit tests ----------------------------------------------─

    /// Helper: insert `n` ordered keys "k00".."kNN" into a dict with the given threshold.
    fn make_hybrid(n: usize, threshold: usize) -> Dict {
        let mut d = Dict::with_small_max_entries(threshold);
        for i in 0..n {
            d.insert(format!("k{i:02}"), Value::from(i as i32));
        }
        d
    }

    /// Returns true if the dict is backed by the Small (Vec) repr.
    fn is_small(d: &Dict) -> bool {
        matches!(d.value, DictRepr::Small(_))
    }

    // -- with_small_max_entries ------------------------------------------------

    #[test]
    fn hybrid_threshold_zero_starts_as_tree() {
        // threshold=0 must produce a Tree-backed dict immediately, before any insert.
        let d = Dict::with_small_max_entries(0);
        assert!(d.is_empty());
        // Inserting into a tree0 dict must still work correctly.
        let mut d = Dict::with_small_max_entries(0);
        d.insert("x".into(), Value::from(1_i32));
        assert_eq!(d.len(), 1);
        assert_eq!(d.get("x"), Some(&Value::from(1_i32)));
    }

    #[test]
    fn hybrid_default_threshold_is_32() {
        let d = Dict::new();
        assert_eq!(d.small_max_entries(), 32);
    }

    // -- insert / spill --------------------------------------------------------

    #[test]
    fn hybrid_insert_stays_small_below_threshold() {
        let d = make_hybrid(8, 16);
        assert!(is_small(&d));
        assert_eq!(d.len(), 8);
    }

    #[test]
    fn hybrid_insert_spills_to_tree_at_threshold() {
        // Filling exactly to threshold keeps Small; one more spills.
        let d = make_hybrid(8, 8);
        assert!(is_small(&d));
        let mut d = make_hybrid(8, 8);
        d.insert("z_extra".into(), Value::from(99_i32));
        // After spill: len is threshold+1 and is_small returns false.
        assert!(!is_small(&d));
        assert_eq!(d.len(), 9);
        assert_eq!(d.get("z_extra"), Some(&Value::from(99_i32)));
    }

    #[test]
    fn hybrid_insert_ordered_fast_path() {
        // Keys inserted in strict ascending order must all be retrievable.
        let d = make_hybrid(16, 32);
        assert!(is_small(&d));
        for i in 0..16_usize {
            assert_eq!(d.get(&format!("k{i:02}")), Some(&Value::from(i as i32)));
        }
    }

    #[test]
    fn hybrid_insert_unordered_keys() {
        let mut d = Dict::with_small_max_entries(8);
        for key in ["e", "a", "c", "b", "d"] {
            d.insert(key.into(), Value::from(1_i32));
        }
        assert_eq!(d.len(), 5);
        // Iteration must be in sorted order.
        let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn hybrid_insert_updates_existing_key_small() {
        let mut d = make_hybrid(4, 8);
        let old = d.insert("k02".into(), Value::from(99_i32));
        assert_eq!(old, Some(Value::from(2_i32)));
        assert_eq!(d.get("k02"), Some(&Value::from(99_i32)));
        assert_eq!(d.len(), 4); // no new entry
    }

    #[test]
    fn hybrid_insert_updates_existing_key_tree() {
        let mut d = make_hybrid(10, 4); // spilled
        let old = d.insert("k02".into(), Value::from(99_i32));
        assert_eq!(old, Some(Value::from(2_i32)));
        assert_eq!(d.get("k02"), Some(&Value::from(99_i32)));
        assert_eq!(d.len(), 10);
    }

    // -- get / get_mut --------------------------------------------------------─

    #[test]
    fn hybrid_get_hit_and_miss_small() {
        let d = make_hybrid(4, 8);
        assert_eq!(d.get("k01"), Some(&Value::from(1_i32)));
        assert!(d.get("missing").is_none());
    }

    #[test]
    fn hybrid_get_hit_and_miss_tree() {
        let d = make_hybrid(10, 4);
        assert_eq!(d.get("k07"), Some(&Value::from(7_i32)));
        assert!(d.get("missing").is_none());
    }

    #[test]
    fn hybrid_get_mut_small() {
        let mut d = make_hybrid(4, 8);
        *d.get_mut("k01").unwrap() = Value::from(42_i32);
        assert_eq!(d.get("k01"), Some(&Value::from(42_i32)));
    }

    // -- remove ----------------------------------------------------------------

    #[test]
    fn hybrid_remove_hit_small() {
        let mut d = make_hybrid(4, 8);
        let removed = d.remove("k02");
        assert_eq!(removed, Some(Value::from(2_i32)));
        assert_eq!(d.len(), 3);
        assert!(d.get("k02").is_none());
        // Remaining keys still accessible and sorted.
        let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["k00", "k01", "k03"]);
    }

    #[test]
    fn hybrid_remove_miss_small() {
        let mut d = make_hybrid(4, 8);
        assert!(d.remove("nope").is_none());
        assert_eq!(d.len(), 4);
    }

    #[test]
    fn hybrid_remove_hit_tree() {
        let mut d = make_hybrid(10, 4);
        let removed = d.remove("k05");
        assert_eq!(removed, Some(Value::from(5_i32)));
        assert_eq!(d.len(), 9);
    }

    // -- clear ----------------------------------------------------------------─

    #[test]
    fn hybrid_clear_small_preserves_small_repr() {
        let mut d = make_hybrid(4, 8);
        d.clear();
        assert!(d.is_empty());
        // Should still be usable as Small after clear.
        d.insert("a".into(), Value::from(1_i32));
        assert_eq!(d.len(), 1);
        assert!(is_small(&d));
    }

    #[test]
    fn hybrid_clear_tree_preserves_tree_repr() {
        let mut d = make_hybrid(10, 4); // spilled to tree
        d.clear();
        assert!(d.is_empty());
        // After clear, threshold=4 dict that was a tree stays tree
        // but can still insert more values.
        d.insert("a".into(), Value::from(1_i32));
        assert_eq!(d.len(), 1);
    }

    // -- shrink_to_fit --------------------------------------------------------─

    #[test]
    fn hybrid_shrink_to_fit_demotes_tree_to_small() {
        let mut d = make_hybrid(10, 8); // spills to tree at 9 entries
        // Remove enough entries to drop ≤ threshold.
        for i in 8..10_usize {
            d.remove(&format!("k{i:02}"));
        }
        assert_eq!(d.len(), 8); // now at threshold
        // Still tree before shrink.
        assert!(!is_small(&d));
        d.shrink_to_fit();
        // Now Small.
        assert!(is_small(&d));
        // All remaining keys intact and sorted.
        let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
        let expected: Vec<String> = (0..8).map(|i| format!("k{i:02}")).collect();
        assert_eq!(
            keys,
            expected.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn hybrid_shrink_to_fit_noop_when_already_small() {
        let mut d = make_hybrid(4, 8);
        assert!(is_small(&d));
        d.shrink_to_fit(); // must not panic and state unchanged
        assert!(is_small(&d));
        assert_eq!(d.len(), 4);
    }

    #[test]
    fn hybrid_shrink_to_fit_noop_when_tree_exceeds_threshold() {
        let mut d = make_hybrid(16, 8); // 16 entries, threshold 8 → stays tree
        assert!(!is_small(&d));
        d.shrink_to_fit();
        // Still tree because len > threshold.
        assert!(!is_small(&d));
        assert_eq!(d.len(), 16);
    }

    // -- pop_first ------------------------------------------------------------─

    #[test]
    fn hybrid_pop_first_small() {
        let mut d = make_hybrid(3, 8);
        let first = d.pop_first();
        assert_eq!(first, Some(("k00".into(), Value::from(0_i32))));
        assert_eq!(d.len(), 2);
    }

    #[test]
    fn hybrid_pop_first_tree() {
        let mut d = make_hybrid(10, 4);
        let first = d.pop_first();
        assert_eq!(first, Some(("k00".into(), Value::from(0_i32))));
        assert_eq!(d.len(), 9);
    }

    #[test]
    fn hybrid_pop_first_empty() {
        let mut d = Dict::new();
        assert!(d.pop_first().is_none());
    }

    // -- FromIterator ----------------------------------------------------------

    #[test]
    fn hybrid_from_iter_small_path() {
        let pairs: Vec<(String, Value)> = (0..8_usize)
            .map(|i| (format!("k{i:02}"), Value::from(i as i32)))
            .collect();
        let d: Dict = pairs.into_iter().collect();
        assert_eq!(d.len(), 8);
        assert!(is_small(&d));
        for i in 0..8_usize {
            assert_eq!(d.get(&format!("k{i:02}")), Some(&Value::from(i as i32)));
        }
    }

    #[test]
    fn hybrid_from_iter_tree_promotion_via_size_hint() {
        // Wrapping in a Vec gives an exact size_hint > 32, so FromIterator
        // should take the Tree-promotion fast path without individual inserts.
        let pairs: Vec<(String, Value)> = (0..64_usize)
            .map(|i| (format!("k{i:02}"), Value::from(i as i32)))
            .collect();
        let d: Dict = pairs.into_iter().collect();
        assert_eq!(d.len(), 64);
        assert!(!is_small(&d));
    }

    // -- From<DictType> --------------------------------------------------------

    #[test]
    fn hybrid_from_dicttype_small_path() {
        use crate::haystack::val::dict::DictType;
        let mut map = DictType::new();
        for i in 0..8_usize {
            map.insert(format!("k{i:02}"), Value::from(i as i32));
        }
        let d = Dict::from(map);
        assert_eq!(d.len(), 8);
        assert!(is_small(&d));
    }

    #[test]
    fn hybrid_from_dicttype_tree_path() {
        use crate::haystack::val::dict::DictType;
        let mut map = DictType::new();
        for i in 0..64_usize {
            map.insert(format!("k{i:02}"), Value::from(i as i32));
        }
        let d = Dict::from(map);
        assert_eq!(d.len(), 64);
        assert!(!is_small(&d));
    }

    // -- size_hint / ExactSizeIterator ----------------------------------------─

    #[test]
    fn hybrid_iter_size_hint_small() {
        let d = make_hybrid(5, 8);
        let mut it = d.iter();
        assert_eq!(it.size_hint(), (5, Some(5)));
        it.next();
        assert_eq!(it.size_hint(), (4, Some(4)));
    }

    #[test]
    fn hybrid_iter_size_hint_tree() {
        let d = make_hybrid(10, 4);
        let it = d.iter();
        assert_eq!(it.size_hint(), (10, Some(10)));
    }

    #[test]
    fn hybrid_iter_exact_size_small() {
        let d = make_hybrid(5, 8);
        assert_eq!(d.iter().len(), 5);
        assert_eq!(d.keys().len(), 5);
        assert_eq!(d.values().len(), 5);
    }

    #[test]
    fn hybrid_iter_exact_size_tree() {
        let d = make_hybrid(10, 4);
        assert_eq!(d.iter().len(), 10);
        assert_eq!(d.keys().len(), 10);
        assert_eq!(d.values().len(), 10);
    }

    #[test]
    fn hybrid_into_iter_exact_size() {
        let d = make_hybrid(5, 8);
        let it = d.into_iter();
        assert_eq!(it.len(), 5);
    }

    // -- ordering invariant ----------------------------------------------------

    #[test]
    fn hybrid_iteration_order_is_always_sorted() {
        // Insert in reverse order to stress the binary-search insertion path.
        let mut d = Dict::with_small_max_entries(16);
        for i in (0..8_usize).rev() {
            d.insert(format!("k{i:02}"), Value::from(i as i32));
        }
        let keys: Vec<String> = d.keys().map(|k| k.clone()).collect();
        let mut expected = keys.clone();
        expected.sort();
        assert_eq!(keys, expected);
    }

    #[test]
    fn hybrid_tree_iteration_order_is_sorted() {
        let d = make_hybrid(16, 4); // threshold=4, so spills
        let keys: Vec<String> = d.keys().map(|k| k.clone()).collect();
        let mut expected = keys.clone();
        expected.sort();
        assert_eq!(keys, expected);
    }
}
