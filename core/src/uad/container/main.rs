use std::{marker::PhantomData, num::Wrapping};

use slab::Slab;

use crate::util::{GetId, Named, RMap};

type Id = u32;
type Key = usize;

pub(crate) struct UadEntityContainer<T, E> {
    counter: Wrapping<Id>,
    pub(super) data: Slab<T>,
    pub(super) id_to_key: RMap<Id, Key>,
    phantom_error: PhantomData<E>,
}
impl<T, E> UadEntityContainer<T, E>
where
    T: GetId<Id> + Named,
    E: From<Id>,
{
    pub(in crate::uad) fn new(capacity: usize) -> Self {
        Self {
            counter: Wrapping(0),
            data: Slab::with_capacity(capacity),
            id_to_key: RMap::with_capacity(capacity),
            phantom_error: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> Id {
        let start = self.counter;
        while self.id_to_key.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of {} ID space", T::get_name());
            }
        }
        let item_id = self.counter.0;
        self.counter += 1;
        item_id
    }
    pub(crate) fn add(&mut self, entity: T) -> Key {
        let id = entity.get_id();
        let key = self.data.insert(entity);
        self.id_to_key.insert(id, key);
        key
    }
    pub(crate) fn key_by_id(&self, id: &Id) -> Option<Key> {
        self.id_to_key.get(id).copied()
    }
    pub(crate) fn key_by_id_err(&self, id: &Id) -> Result<Key, E> {
        match self.id_to_key.get(id) {
            Some(key) => Ok(*key),
            None => Err(E::from(*id)),
        }
    }
    pub(crate) fn id_by_key(&self, key: Key) -> Id {
        self.get(key).get_id()
    }
    pub(crate) fn try_get(&self, key: Key) -> Option<&T> {
        self.data.get(key)
    }
    pub(crate) fn get(&self, key: Key) -> &T {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(key).unwrap()
    }
    pub(crate) fn get_mut(&mut self, key: Key) -> &mut T {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(key).unwrap()
    }
    pub(crate) fn get2_mut(&mut self, key1: Key, key2: Key) -> (&mut T, &mut T) {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get2_mut(key1, key2).unwrap()
    }
    pub(crate) fn get_disjoint_mut<const N: Key>(&mut self, keys: [Key; N]) -> [&mut T; N] {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_disjoint_mut(keys).unwrap()
    }
    pub(crate) fn remove(&mut self, key: Key) -> T {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(key);
        self.id_to_key.remove(&entity.get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (Key, &T)> {
        self.id_to_key.values().map(|&key| (key, self.data.get(key).unwrap()))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (Key, &mut T)> {
        self.data.iter_mut()
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = Key> {
        self.id_to_key.values().copied()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.id_to_key.values().map(|&key| self.data.get(key).unwrap())
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, entity)| entity)
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}
impl<T, E> Clone for UadEntityContainer<T, E>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            counter: self.counter,
            data: self.data.clone(),
            id_to_key: self.id_to_key.clone(),
            phantom_error: PhantomData,
        }
    }
}
