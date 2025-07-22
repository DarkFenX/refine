use slab::Slab;

use crate::util::{GetId, Named, RMap};

type Key = usize;

pub(crate) struct REntityContainer<I, T> {
    data: Slab<T>,
    id_to_key: RMap<I, Key>,
}
impl<I, T> REntityContainer<I, T>
where
    I: Eq + std::hash::Hash,
    T: GetId<I> + Named,
{
    pub(crate) fn new() -> Self {
        Self {
            data: Slab::new(),
            id_to_key: RMap::new(),
        }
    }

    pub(crate) fn add(&mut self, entity: T) -> Key {
        let id = entity.get_id();
        let key = self.data.insert(entity);
        self.id_to_key.insert(id, key);
        key
    }
    pub(crate) fn key_by_id(&self, id: &I) -> Option<Key> {
        self.id_to_key.get(id).copied()
    }
    pub(crate) fn id_by_key(&self, key: Key) -> I {
        self.get(key).get_id()
    }
    pub(crate) fn try_get(&self, key: Key) -> Option<&T> {
        self.data.get(key)
    }
    pub(crate) fn get(&self, key: Key) -> &T {
        self.data.get(key).unwrap()
    }
    pub(crate) fn get_mut(&mut self, key: Key) -> &mut T {
        self.data.get_mut(key).unwrap()
    }
    pub(crate) fn remove(&mut self, key: Key) -> T {
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
