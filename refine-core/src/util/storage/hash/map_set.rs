use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use itertools::Either;

use super::{
    map::{Map, RMap},
    set::{RSet, Set},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-const-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) type RMapRSet<K, V> = MapSet<K, V, rustc_hash::FxBuildHasher, rustc_hash::FxBuildHasher>;
impl<K, V> Default for RMapRSet<K, V> {
    fn default() -> Self {
        Self {
            data: RMap::new(),
            buffer: RSet::new(),
        }
    }
}
impl<K, V> RMapRSet<K, V> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
impl<K, V> RMapRSet<K, V>
where
    K: Eq + Hash,
    V: Eq + Hash,
{
    pub(crate) fn add_entry(&mut self, key: K, value: V) {
        match self.data.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(Values::One(value));
            }
            Entry::Occupied(mut entry) => entry.get_mut().insert(value),
        }
    }
    pub(crate) fn extend_entries(&mut self, key: K, mut entries: impl ExactSizeIterator<Item = V>) {
        match entries.len() {
            0 => (),
            1 => self.add_entry(key, entries.next().unwrap()),
            _ => match self.data.entry(key) {
                Entry::Vacant(entry) => {
                    let mut set = RSet::with_capacity(entries.len());
                    set.extend(entries);
                    entry.insert(Values::Many(set));
                }
                Entry::Occupied(mut entry) => entry.get_mut().extend(entries),
            },
        }
    }
    pub(crate) fn remove_key(&mut self, key: &K) -> impl ExactSizeIterator<Item = V> + use<K, V> {
        match self.data.remove(key) {
            Some(values) => values.into_iter(),
            None => Either::Left(None.into_iter()),
        }
    }
}
impl<V> Values<V, rustc_hash::FxBuildHasher>
where
    V: Eq + Hash,
{
    fn insert(&mut self, value: V) {
        match self {
            Self::One(old_value) if *old_value == value => (),
            Self::One(_) => {
                let Self::One(old_value) = std::mem::replace(self, Self::Many(RSet::with_capacity(2))) else {
                    unreachable!()
                };
                if let Self::Many(set) = self {
                    set.insert(old_value);
                    set.insert(value);
                }
            }
            Self::Many(set) => {
                set.insert(value);
            }
        }
    }
    fn extend(&mut self, values: impl ExactSizeIterator<Item = V>) {
        match self {
            Self::One(_) => {
                let set = RSet::with_capacity(values.len() + 1);
                let Self::One(old_value) = std::mem::replace(self, Self::Many(set)) else {
                    unreachable!()
                };
                if let Self::Many(set) = self {
                    set.insert(old_value);
                    set.extend(values);
                }
            }
            Self::Many(set) => set.extend(values),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct MapSet<K, V, H1, H2> {
    data: Map<K, Values<V, H2>, H1>,
    buffer: Set<V, H2>,
}
impl<K, V, H1, H2> MapSet<K, V, H1, H2>
where
    K: Eq + Hash,
    V: Eq + Hash,
    H1: BuildHasher,
    H2: BuildHasher,
{
    pub(crate) fn get(&self, key: &K) -> impl ExactSizeIterator<Item = &V> + use<'_, K, V, H1, H2> {
        match self.data.get(key) {
            Some(values) => values.iter(),
            None => Either::Left(None.into_iter()),
        }
    }
    pub(crate) fn extend_vec(&self, vec: &mut Vec<V>, key: &K)
    where
        V: Copy,
    {
        vec.extend(self.get(key).copied());
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (&K, impl ExactSizeIterator<Item = &V>)> {
        self.data.iter().map(|(k, v)| (k, v.iter()))
    }
    #[expect(dead_code)]
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = &K> {
        self.data.keys()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &V>> {
        self.data.values().map(|v| v.iter())
    }
    pub(crate) fn contains_entry(&self, key: &K, value: &V) -> bool {
        match self.data.get(key) {
            Some(values) => values.contains(value),
            None => false,
        }
    }
    #[expect(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn remove_entry(&mut self, key: K, value: &V) {
        if let Entry::Occupied(mut entry) = self.data.entry(key) {
            let emptied = match entry.get_mut() {
                Values::One(old_value) => old_value == value,
                Values::Many(set) => {
                    set.remove(value);
                    set.is_empty()
                }
            };
            if emptied {
                entry.remove();
            }
        }
    }
    // Buffer methods
    pub(crate) fn buffer_if<F>(&mut self, key: K, mut filter: F)
    where
        F: FnMut(&V) -> bool,
    {
        if let Entry::Occupied(mut entry) = self.data.entry(key) {
            let emptied = match entry.get_mut() {
                Values::One(value) => filter(value),
                Values::Many(set) => {
                    self.buffer.extend(set.extract_if(filter));
                    set.is_empty()
                }
            };
            if emptied && let Values::One(value) = entry.remove() {
                self.buffer.insert(value);
            }
        }
    }
    pub(crate) fn drain_buffer(&mut self) -> impl ExactSizeIterator<Item = V> {
        self.buffer.drain()
    }
}

#[derive(Clone)]
enum Values<V, H> {
    One(V),
    Many(Set<V, H>),
}
impl<V, H> Values<V, H>
where
    V: Eq + Hash,
    H: BuildHasher,
{
    fn iter(&self) -> Either<std::option::IntoIter<&V>, impl ExactSizeIterator<Item = &V>> {
        match self {
            Self::One(value) => Either::Left(Some(value).into_iter()),
            Self::Many(set) => Either::Right(set.iter()),
        }
    }
    fn contains(&self, value: &V) -> bool {
        match self {
            Self::One(old_value) => old_value == value,
            Self::Many(set) => set.contains(value),
        }
    }
}
impl<V, H> IntoIterator for Values<V, H> {
    type Item = V;
    type IntoIter = Either<std::option::IntoIter<V>, std::collections::hash_set::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::One(value) => Either::Left(Some(value).into_iter()),
            Self::Many(set) => Either::Right(set.into_iter()),
        }
    }
}
