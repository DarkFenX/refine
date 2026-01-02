use std::{hash::Hash, marker::PhantomData, num::Wrapping};

use slab::Slab;

use crate::util::{GetId, Named, RMap};

pub(crate) struct UEntityContainer<T, Key, Id, Err> {
    counter: Wrapping<Id>,
    pub(super) data: Slab<T>,
    pub(super) id_to_key: RMap<Id, usize>,
    phantom_key: PhantomData<Key>,
    phantom_error: PhantomData<Err>,
}
impl<T, Key, Id, Err> UEntityContainer<T, Key, Id, Err>
where
    T: GetId<Id> + Named,
    Key: Copy + Into<usize> + From<usize>,
    Id: Copy + Default + Eq + Hash,
    Wrapping<Id>: std::ops::AddAssign<u32>,
    Err: From<Id>,
{
    pub(in crate::ud) fn new(capacity: usize) -> Self {
        Self {
            counter: Wrapping(Id::default()),
            data: Slab::with_capacity(capacity),
            id_to_key: RMap::with_capacity(capacity),
            phantom_key: PhantomData,
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
        let ikey = self.data.insert(entity);
        self.id_to_key.insert(id, ikey);
        ikey.into()
    }
    pub(crate) fn key_by_id(&self, id: &Id) -> Option<Key> {
        self.id_to_key.get(id).map(|&ikey| ikey.into())
    }
    pub(crate) fn key_by_id_err(&self, id: &Id) -> Result<Key, Err> {
        match self.id_to_key.get(id) {
            Some(&ikey) => Ok(ikey.into()),
            None => Err(Err::from(*id)),
        }
    }
    pub(crate) fn id_by_key(&self, key: Key) -> Id {
        self.get(key).get_id()
    }
    pub(crate) fn try_get(&self, key: Key) -> Option<&T> {
        self.data.get(key.into())
    }
    pub(crate) fn get(&self, key: Key) -> &T {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(key.into()).unwrap()
    }
    pub(crate) fn get_mut(&mut self, key: Key) -> &mut T {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(key.into()).unwrap()
    }
    pub(crate) fn remove(&mut self, key: Key) -> T {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(key.into());
        self.id_to_key.remove(&entity.get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (Key, &T)> {
        self.id_to_key
            .values()
            .map(|&ikey| (ikey.into(), self.data.get(ikey).unwrap()))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (Key, &mut T)> {
        self.data.iter_mut().map(|(ikey, entity)| (ikey.into(), entity))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = Key> {
        self.id_to_key.values().map(|&ikey| ikey.into())
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.id_to_key.values().map(|&ikey| self.data.get(ikey).unwrap())
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, entity)| entity)
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}
impl<T, Key, Id, Err> Clone for UEntityContainer<T, Key, Id, Err>
where
    T: Clone,
    Id: Copy,
{
    fn clone(&self) -> Self {
        Self {
            counter: self.counter,
            data: self.data.clone(),
            id_to_key: self.id_to_key.clone(),
            phantom_key: PhantomData,
            phantom_error: PhantomData,
        }
    }
}
