// Copyright (C) 2020 - 2026, J2 Innovations

//! A generic hybrid small-vector / tree-map backing store.
//!
//! [`HybridMap`] factors out the storage strategy used internally by
//! [`Dict`](crate::val::Dict): entries are kept in a sorted `Vec` while the
//! map stays small (avoiding a heap-heavy tree allocation for the common
//! case of a handful of tags), and are only promoted to a heavier ordered
//! map (by default a [`BTreeMap`]) once the entry count exceeds a
//! configurable threshold.
//!
//! Projects that wrap [`Value`](crate::val::Value) with their own extra
//! variants (and so can't reuse `Dict` itself) can reuse this type directly
//! with their own key/value/map types instead of reimplementing the hybrid
//! storage strategy, e.g. `HybridMap<String, MyValue>`.

use std::borrow::Borrow;
use std::collections::BTreeMap;

/// Trait implemented by the "heavy" backing map used as the fallback tier of
/// a [`HybridMap`] once its entry count exceeds the small-vector threshold.
///
/// Implemented out of the box for [`BTreeMap<K, V>`]; other ordered-map
/// implementations can implement this trait to plug into a `HybridMap`.
pub trait TreeMap<K, V>: Default + FromIterator<(K, V)> {
    /// Iterator returned by [`TreeMap::iter`].
    type Iter<'a>: Iterator<Item = (&'a K, &'a V)> + ExactSizeIterator
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    /// Iterator returned by [`TreeMap::iter_mut`].
    type IterMut<'a>: Iterator<Item = (&'a K, &'a mut V)> + ExactSizeIterator
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    /// Iterator returned by [`TreeMap::into_iter`].
    type IntoIter: Iterator<Item = (K, V)> + ExactSizeIterator;

    /// Returns the number of entries in the map.
    fn len(&self) -> usize;

    /// Returns `true` if the map contains no entries.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes all entries from the map.
    fn clear(&mut self);

    /// Inserts a key-value pair, returning the previous value for `key`, if any.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Removes and returns the first (lowest-keyed) entry, if any.
    fn pop_first(&mut self) -> Option<(K, V)>;

    /// Retains only the entries for which the predicate returns `true`.
    fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F);

    /// Returns a reference to the value for `key`, if present.
    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized;

    /// Returns a mutable reference to the value for `key`, if present.
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized;

    /// Removes `key` from the map, returning its value if it was present.
    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized;

    /// Returns an iterator over `(&K, &V)` pairs, in key order.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns an iterator over `(&K, &mut V)` pairs, in key order.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    /// Converts the map into an owning iterator over `(K, V)` pairs, in key order.
    fn into_iter(self) -> Self::IntoIter;
}

impl<K: Ord, V> TreeMap<K, V> for BTreeMap<K, V> {
    type Iter<'a>
        = std::collections::btree_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;
    type IterMut<'a>
        = std::collections::btree_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn pop_first(&mut self) -> Option<(K, V)> {
        self.pop_first()
    }

    fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F) {
        self.retain(f);
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get(key)
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_mut(key)
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.remove(key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

#[derive(Clone, Debug)]
enum HybridRepr<K, V, M> {
    Small(Vec<(K, V)>),
    // Boxed so the discriminant can be packed into `Vec`'s pointer niche.
    Tree(Box<M>),
}

/// A hybrid map: a sorted small-vector back-store for tiny maps, spilling
/// over to a heavier ordered map `M` (a [`BTreeMap<K, V>`] by default) once
/// the entry count exceeds a threshold.
///
/// Iteration is always in key order, regardless of the active back-store.
#[derive(Clone, Debug)]
pub struct HybridMap<K, V, M = BTreeMap<K, V>> {
    repr: HybridRepr<K, V, M>,
    small_max_entries: usize,
}

impl<K, V, M> HybridMap<K, V, M>
where
    K: Ord,
    M: TreeMap<K, V>,
{
    /// Default threshold for the small-vector back-store.
    pub const DEFAULT_SMALL_MAX_ENTRIES: usize = 32;

    /// Construct a new `HybridMap` with the default small-store threshold.
    pub fn new() -> Self {
        Self::with_small_max_entries(Self::DEFAULT_SMALL_MAX_ENTRIES)
    }

    /// Construct a new `HybridMap` with a custom small-store threshold.
    /// If `small_max_entries` is 0, the small-vector back-store is disabled
    /// and the map will use the `M` representation right away.
    pub fn with_small_max_entries(small_max_entries: usize) -> Self {
        let repr = if small_max_entries == 0 {
            HybridRepr::Tree(Box::default())
        } else {
            HybridRepr::Small(Vec::new())
        };
        HybridMap {
            repr,
            small_max_entries,
        }
    }

    /// Return the active small-store threshold for this map.
    #[inline]
    pub fn small_max_entries(&self) -> usize {
        self.small_max_entries
    }

    /// True if this map is currently backed by the small-vector store.
    #[inline]
    pub fn is_small(&self) -> bool {
        matches!(self.repr, HybridRepr::Small(_))
    }

    fn small_search<Q>(entries: &[(K, V)], key: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        entries.binary_search_by(|(k, _)| k.borrow().cmp(key))
    }

    fn spill_to_tree(&mut self) {
        if let HybridRepr::Small(entries) = &mut self.repr {
            let map = entries.drain(..).collect::<M>();
            self.repr = HybridRepr::Tree(Box::new(map));
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match &self.repr {
            HybridRepr::Small(entries) => entries.len(),
            HybridRepr::Tree(map) => map.len(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        match &mut self.repr {
            HybridRepr::Small(entries) => entries.clear(),
            HybridRepr::Tree(map) => map.clear(),
        }
    }

    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &self.repr {
            HybridRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| &entries[pos].1),
            HybridRepr::Tree(map) => map.get(key),
        }
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &mut self.repr {
            HybridRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| &mut entries[pos].1),
            HybridRepr::Tree(map) => map.get_mut(key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.repr {
            HybridRepr::Small(entries) => {
                if entries.len() < self.small_max_entries
                    && entries.last().is_none_or(|(last_key, _)| &key > last_key)
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
                            match &mut self.repr {
                                HybridRepr::Tree(map) => map.insert(key, value),
                                HybridRepr::Small(_) => None,
                            }
                        }
                    }
                }
            }
            HybridRepr::Tree(map) => map.insert(key, value),
        }
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &mut self.repr {
            HybridRepr::Small(entries) => Self::small_search(entries, key)
                .ok()
                .map(|pos| entries.remove(pos).1),
            HybridRepr::Tree(map) => map.remove(key),
        }
    }

    #[inline]
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        match &mut self.repr {
            HybridRepr::Small(entries) => {
                if entries.is_empty() {
                    None
                } else {
                    Some(entries.remove(0))
                }
            }
            HybridRepr::Tree(map) => map.pop_first(),
        }
    }

    /// Demote a tree-backed map back to the small-vector store once its
    /// entry count has dropped to at or below the small-store threshold.
    ///
    /// This is the inverse of the automatic spill that happens in
    /// [`insert`](Self::insert). If the map is already small-backed this is
    /// a no-op.
    pub fn shrink_to_fit(&mut self) {
        if let HybridRepr::Tree(map) = &mut self.repr
            && map.len() <= self.small_max_entries
        {
            // Take ownership without cloning; `M: Default` leaves a valid empty map behind.
            let owned = std::mem::take(map.as_mut());
            self.repr = HybridRepr::Small(owned.into_iter().collect());
        }
    }

    /// Retains only the entries for which the predicate returns `true`.
    ///
    /// When called on a tree-backed map and the surviving entry count drops
    /// to or below the small-store threshold, the storage is automatically
    /// downgraded back to the small-vector representation.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        match &mut self.repr {
            HybridRepr::Small(entries) => entries.retain_mut(|(k, v)| f(k, v)),
            HybridRepr::Tree(map) => map.retain(|k, v| f(k, v)),
        }
        self.shrink_to_fit();
    }

    #[inline]
    pub fn iter(&self) -> HybridIter<'_, K, V, M> {
        match &self.repr {
            HybridRepr::Small(entries) => HybridIter::Small(entries.iter()),
            HybridRepr::Tree(map) => HybridIter::Tree(map.iter()),
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> HybridIterMut<'_, K, V, M> {
        match &mut self.repr {
            HybridRepr::Small(entries) => HybridIterMut::Small(entries.iter_mut()),
            HybridRepr::Tree(map) => HybridIterMut::Tree(map.iter_mut()),
        }
    }

    #[inline]
    pub fn keys(&self) -> HybridKeys<'_, K, V, M> {
        HybridKeys { inner: self.iter() }
    }

    #[inline]
    pub fn values(&self) -> HybridValues<'_, K, V, M> {
        HybridValues { inner: self.iter() }
    }

    #[inline]
    pub fn values_mut(&mut self) -> HybridValuesMut<'_, K, V, M> {
        HybridValuesMut {
            inner: self.iter_mut(),
        }
    }

    /// Returns `None` when the size hint signals the entry count will exceed
    /// the small-vec threshold (callers should build a tree-backed map
    /// directly), or `Some(map)` with a small-backed map pre-allocated to
    /// the hinted capacity.
    fn prepare_from_hint(lower: usize, upper: Option<usize>) -> Option<Self> {
        if lower > Self::DEFAULT_SMALL_MAX_ENTRIES
            || upper.is_some_and(|upper| upper > Self::DEFAULT_SMALL_MAX_ENTRIES)
        {
            return None;
        }
        let mut map = Self::new();
        if lower > 0
            && let HybridRepr::Small(entries) = &mut map.repr
        {
            entries.reserve(lower.min(map.small_max_entries));
        }
        Some(map)
    }

    /// Constructs a `HybridMap` from a fallible iterator of `(K, V)` pairs.
    ///
    /// Applies the same size-hint optimisation as [`FromIterator`]: when the
    /// iterator reports more than `small_max_entries` items the backing
    /// store starts as a tree directly, skipping the small-vec stage.
    ///
    /// The first `Err` item short-circuits collection and is returned
    /// immediately, leaving any remaining items unconsumed.
    pub fn try_from_iter<E, I>(iter: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<(K, V), E>>,
    {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();

        let Some(mut map) = Self::prepare_from_hint(lower, upper) else {
            let tree = iter.collect::<Result<M, E>>()?;
            return Ok(HybridMap {
                repr: HybridRepr::Tree(Box::new(tree)),
                small_max_entries: Self::DEFAULT_SMALL_MAX_ENTRIES,
            });
        };

        for result in iter {
            let (k, v) = result?;
            map.insert(k, v);
        }
        Ok(map)
    }
}

impl<K, V, M> Default for HybridMap<K, V, M>
where
    K: Ord,
    M: TreeMap<K, V>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Implement FromIterator for `HybridMap`
impl<K, V, M> FromIterator<(K, V)> for HybridMap<K, V, M>
where
    K: Ord,
    M: TreeMap<K, V>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();

        let Some(mut map) = Self::prepare_from_hint(lower, upper) else {
            return HybridMap {
                repr: HybridRepr::Tree(Box::new(iter.collect())),
                small_max_entries: Self::DEFAULT_SMALL_MAX_ENTRIES,
            };
        };

        for (k, v) in iter.by_ref() {
            map.insert(k, v);
        }
        map
    }
}

/// Converts from the heavy map type `M` to a `HybridMap`
impl<K, V, M> From<M> for HybridMap<K, V, M>
where
    K: Ord,
    M: TreeMap<K, V>,
{
    fn from(from: M) -> Self {
        let small_max_entries = Self::DEFAULT_SMALL_MAX_ENTRIES;
        if from.len() <= small_max_entries {
            HybridMap {
                repr: HybridRepr::Small(from.into_iter().collect()),
                small_max_entries,
            }
        } else {
            HybridMap {
                repr: HybridRepr::Tree(Box::new(from)),
                small_max_entries,
            }
        }
    }
}

impl<K, V, M> HybridMap<K, V, M>
where
    K: Ord,
    M: TreeMap<K, V>,
{
    /// Converts this `HybridMap` back into the heavy map type `M`.
    ///
    /// A free-standing `From<HybridMap<K, V, M>> for M` impl isn't possible
    /// here since `M` is a type parameter, not a local type (orphan rules).
    pub fn into_inner(self) -> M {
        match self.repr {
            HybridRepr::Small(entries) => entries.into_iter().collect(),
            HybridRepr::Tree(map) => *map,
        }
    }
}

pub enum HybridIter<'a, K: 'a, V: 'a, M: TreeMap<K, V> + 'a> {
    Small(std::slice::Iter<'a, (K, V)>),
    Tree(M::Iter<'a>),
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> Iterator for HybridIter<'a, K, V, M> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridIter::Small(iter) => iter.next().map(|(k, v)| (k, v)),
            HybridIter::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            HybridIter::Small(iter) => iter.size_hint(),
            HybridIter::Tree(iter) => iter.size_hint(),
        }
    }
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> ExactSizeIterator for HybridIter<'a, K, V, M> {}

pub enum HybridIterMut<'a, K: 'a, V: 'a, M: TreeMap<K, V> + 'a> {
    Small(std::slice::IterMut<'a, (K, V)>),
    Tree(M::IterMut<'a>),
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> Iterator for HybridIterMut<'a, K, V, M> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridIterMut::Small(iter) => iter.next().map(|(k, v)| (&*k, v)),
            HybridIterMut::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            HybridIterMut::Small(iter) => iter.size_hint(),
            HybridIterMut::Tree(iter) => iter.size_hint(),
        }
    }
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> ExactSizeIterator for HybridIterMut<'a, K, V, M> {}

pub enum HybridIntoIter<K, V, M: TreeMap<K, V>> {
    Small(std::vec::IntoIter<(K, V)>),
    Tree(M::IntoIter),
}

impl<K, V, M: TreeMap<K, V>> Iterator for HybridIntoIter<K, V, M> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridIntoIter::Small(iter) => iter.next(),
            HybridIntoIter::Tree(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            HybridIntoIter::Small(iter) => iter.size_hint(),
            HybridIntoIter::Tree(iter) => iter.size_hint(),
        }
    }
}

impl<K, V, M: TreeMap<K, V>> ExactSizeIterator for HybridIntoIter<K, V, M> {}

impl<K, V, M: TreeMap<K, V>> IntoIterator for HybridMap<K, V, M> {
    type Item = (K, V);
    type IntoIter = HybridIntoIter<K, V, M>;

    fn into_iter(self) -> Self::IntoIter {
        match self.repr {
            HybridRepr::Small(entries) => HybridIntoIter::Small(entries.into_iter()),
            HybridRepr::Tree(map) => HybridIntoIter::Tree(map.into_iter()),
        }
    }
}

impl<'a, K: Ord, V, M: TreeMap<K, V> + 'a> IntoIterator for &'a HybridMap<K, V, M> {
    type Item = (&'a K, &'a V);
    type IntoIter = HybridIter<'a, K, V, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: Ord, V, M: TreeMap<K, V> + 'a> IntoIterator for &'a mut HybridMap<K, V, M> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = HybridIterMut<'a, K, V, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct HybridKeys<'a, K: 'a, V: 'a, M: TreeMap<K, V> + 'a> {
    inner: HybridIter<'a, K, V, M>,
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> Iterator for HybridKeys<'a, K, V, M> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> ExactSizeIterator for HybridKeys<'a, K, V, M> {}

pub struct HybridValues<'a, K: 'a, V: 'a, M: TreeMap<K, V> + 'a> {
    inner: HybridIter<'a, K, V, M>,
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> Iterator for HybridValues<'a, K, V, M> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> ExactSizeIterator for HybridValues<'a, K, V, M> {}

pub struct HybridValuesMut<'a, K: 'a, V: 'a, M: TreeMap<K, V> + 'a> {
    inner: HybridIterMut<'a, K, V, M>,
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> Iterator for HybridValuesMut<'a, K, V, M> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V, M: TreeMap<K, V> + 'a> ExactSizeIterator for HybridValuesMut<'a, K, V, M> {}

#[cfg(test)]
mod test {
    use super::*;

    /// A minimal custom `TreeMap` impl (a sorted `Vec`, distinct from
    /// `BTreeMap`) used to verify that `HybridMap`/`TreeMap` are genuinely
    /// generic over the heavy map type, not implicitly tied to `BTreeMap`.
    #[derive(Clone, Debug)]
    struct SortedVecMap<K, V>(Vec<(K, V)>);

    impl<K, V> Default for SortedVecMap<K, V> {
        fn default() -> Self {
            SortedVecMap(Vec::new())
        }
    }

    impl<K: Ord, V> FromIterator<(K, V)> for SortedVecMap<K, V> {
        fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
            let mut entries: Vec<(K, V)> = iter.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            SortedVecMap(entries)
        }
    }

    impl<K: Ord, V> TreeMap<K, V> for SortedVecMap<K, V> {
        type Iter<'a>
            = std::iter::Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>
        where
            K: 'a,
            V: 'a;
        type IterMut<'a>
            = std::iter::Map<
            std::slice::IterMut<'a, (K, V)>,
            fn(&'a mut (K, V)) -> (&'a K, &'a mut V),
        >
        where
            K: 'a,
            V: 'a;
        type IntoIter = std::vec::IntoIter<(K, V)>;

        fn len(&self) -> usize {
            self.0.len()
        }

        fn clear(&mut self) {
            self.0.clear();
        }

        fn insert(&mut self, key: K, value: V) -> Option<V> {
            match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
                Ok(pos) => Some(std::mem::replace(&mut self.0[pos].1, value)),
                Err(pos) => {
                    self.0.insert(pos, (key, value));
                    None
                }
            }
        }

        fn pop_first(&mut self) -> Option<(K, V)> {
            if self.0.is_empty() {
                None
            } else {
                Some(self.0.remove(0))
            }
        }

        fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, mut f: F) {
            self.0.retain_mut(|(k, v)| f(k, v));
        }

        fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
        {
            self.0
                .binary_search_by(|(k, _)| k.borrow().cmp(key))
                .ok()
                .map(|pos| &self.0[pos].1)
        }

        fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
        {
            match self.0.binary_search_by(|(k, _)| k.borrow().cmp(key)) {
                Ok(pos) => Some(&mut self.0[pos].1),
                Err(_) => None,
            }
        }

        fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where
            K: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
        {
            self.0
                .binary_search_by(|(k, _)| k.borrow().cmp(key))
                .ok()
                .map(|pos| self.0.remove(pos).1)
        }

        fn iter(&self) -> Self::Iter<'_> {
            self.0.iter().map(|(k, v)| (k, v))
        }

        fn iter_mut(&mut self) -> Self::IterMut<'_> {
            self.0.iter_mut().map(|(k, v)| (&*k, v))
        }

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    /// Insert `n` ordered keys "k00".."kNN" into a `HybridMap` backed by
    /// `SortedVecMap` with the given threshold.
    fn make_hybrid(
        n: usize,
        threshold: usize,
    ) -> HybridMap<String, i32, SortedVecMap<String, i32>> {
        let mut map = HybridMap::with_small_max_entries(threshold);
        for i in 0..n {
            map.insert(format!("k{i:02}"), i as i32);
        }
        map
    }

    #[test]
    fn custom_tree_map_stays_small_below_threshold() {
        let map = make_hybrid(4, 8);
        assert!(map.is_small());
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn custom_tree_map_spills_to_tree_at_threshold() {
        let mut map = make_hybrid(8, 8);
        assert!(map.is_small());
        map.insert("z_extra".into(), 99);
        assert!(!map.is_small());
        assert_eq!(map.len(), 9);
        assert_eq!(map.get("z_extra"), Some(&99));
    }

    #[test]
    fn custom_tree_map_get_remove_and_ordering() {
        let mut map = make_hybrid(10, 4); // spills to SortedVecMap
        assert!(!map.is_small());
        assert_eq!(map.get("k05"), Some(&5));
        assert_eq!(map.remove("k05"), Some(5));
        assert!(map.get("k05").is_none());
        assert_eq!(map.len(), 9);

        let keys: Vec<&String> = map.keys().collect();
        let mut expected = keys.clone();
        expected.sort();
        assert_eq!(keys, expected);
    }

    #[test]
    fn custom_tree_map_shrink_to_fit_round_trip() {
        let mut map = make_hybrid(10, 8); // spills at 9 entries
        for i in 8..10 {
            map.remove(&format!("k{i:02}"));
        }
        assert!(!map.is_small());
        map.shrink_to_fit();
        assert!(map.is_small());
        assert_eq!(map.len(), 8);
    }

    #[test]
    fn custom_tree_map_into_inner_round_trip() {
        let map = make_hybrid(20, 4); // tree-backed
        let inner: SortedVecMap<String, i32> = map.into_inner();
        assert_eq!(inner.len(), 20);
        assert_eq!(inner.get("k10"), Some(&10));
    }
}
