use std::{
    collections::hash_map::Entry,
    hash::{BuildHasher, Hash},
};

use crate::util::storage::Map;

pub(crate) type SmallRSet<V, const N: usize> = SmallSet<V, N, rustc_hash::FxBuildHasher>;

pub(crate) struct SmallSet<V, const N: usize, H> {
    len: usize,
    inline: [V; N],
    spill: Option<Box<Spilled<V, H>>>,
}
#[derive(Clone)]
struct Spilled<V, H> {
    vals: Vec<V>,
    index: Map<V, usize, H>,
}
impl<V, const N: usize, H> Default for SmallSet<V, N, H>
where
    V: Copy + Default,
{
    fn default() -> Self {
        Self {
            len: 0,
            inline: [V::default(); N],
            spill: None,
        }
    }
}
impl<V, const N: usize, H> Clone for SmallSet<V, N, H>
where
    V: Copy,
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            inline: self.inline,
            spill: match self.len > N {
                true => self.spill.clone(),
                false => None,
            },
        }
    }
}
impl<V, const N: usize, H> SmallSet<V, N, H>
where
    V: Copy + Default + Eq + Hash,
    H: BuildHasher + Default,
{
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, V> {
        match self.len > N {
            true => self.spill.as_ref().unwrap().vals.iter(),
            false => self.inline[..self.len].iter(),
        }
    }
    pub(crate) fn contains(&self, val: &V) -> bool {
        match self.len > N {
            true => self.spill.as_ref().unwrap().index.contains_key(val),
            false => self.inline[..self.len].contains(val),
        }
    }
    pub(crate) fn len(&self) -> usize {
        self.len
    }
    // Modification methods
    pub(crate) fn insert(&mut self, val: V) -> bool {
        if self.len < N {
            if self.inline[..self.len].contains(&val) {
                return false;
            }
            self.inline[self.len] = val;
            self.len += 1;
            return true;
        }
        if self.len == N {
            if self.inline.contains(&val) {
                return false;
            }
            let spill = self.spill.get_or_insert_with(|| {
                Box::new(Spilled {
                    vals: Vec::with_capacity(N * 2),
                    index: Map::with_hasher(H::default()),
                })
            });
            spill.vals.clear();
            spill.vals.extend_from_slice(&self.inline);
            spill.vals.push(val);
            spill.index.clear();
            spill.index.extend(spill.vals.iter().enumerate().map(|(i, &v)| (v, i)));
            self.len += 1;
            return true;
        }
        let spill = self.spill.as_mut().unwrap();
        match spill.index.entry(val) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(spill.vals.len());
                spill.vals.push(val);
                self.len += 1;
                true
            }
        }
    }
    pub(crate) fn extend(&mut self, vals: impl IntoIterator<Item = V>) {
        for val in vals {
            self.insert(val);
        }
    }
    pub(crate) fn remove(&mut self, val: &V) -> bool {
        if self.len <= N {
            let Some(pos) = self.inline[..self.len].iter().position(|v| v == val) else {
                return false;
            };
            self.len -= 1;
            self.inline[pos] = self.inline[self.len];
            return true;
        }
        let spill = self.spill.as_mut().unwrap();
        let Some(pos) = spill.index.remove(val) else {
            return false;
        };
        spill.vals.swap_remove(pos);
        self.len -= 1;
        if self.len == N {
            self.inline.copy_from_slice(&spill.vals);
        } else if let Some(&moved) = spill.vals.get(pos) {
            spill.index.insert(moved, pos);
        }
        true
    }
    pub(crate) fn drain(&mut self) -> impl ExactSizeIterator<Item = V> {
        let len = std::mem::take(&mut self.len);
        match len > N {
            true => self.spill.as_ref().unwrap().vals.iter().copied(),
            false => self.inline[..len].iter().copied(),
        }
    }
}
