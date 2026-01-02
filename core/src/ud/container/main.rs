use std::{hash::Hash, marker::PhantomData, num::Wrapping};

use slab::Slab;

use crate::util::{GetId, Named, RMap};

pub(crate) struct UEntityContainer<T, ExtId, IntId, Err> {
    counter: Wrapping<ExtId>,
    pub(super) data: Slab<T>,
    pub(super) ext_id_to_slab_key: RMap<ExtId, usize>,
    phantom_int: PhantomData<IntId>,
    phantom_error: PhantomData<Err>,
}
impl<T, ExtId, IntId, Err> UEntityContainer<T, ExtId, IntId, Err>
where
    T: GetId<ExtId> + Named,
    IntId: Copy + Into<usize> + From<usize>,
    ExtId: Copy + Default + Eq + Hash,
    Wrapping<ExtId>: std::ops::AddAssign<u32>,
    Err: From<ExtId>,
{
    pub(in crate::ud) fn new(capacity: usize) -> Self {
        Self {
            counter: Wrapping(ExtId::default()),
            data: Slab::with_capacity(capacity),
            ext_id_to_slab_key: RMap::with_capacity(capacity),
            phantom_int: PhantomData,
            phantom_error: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> ExtId {
        let start = self.counter;
        while self.ext_id_to_slab_key.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of {} ID space", T::get_name());
            }
        }
        let item_id = self.counter.0;
        self.counter += 1;
        item_id
    }
    pub(crate) fn add(&mut self, entity: T) -> IntId {
        let ext_id = entity.get_id();
        let slab_key = self.data.insert(entity);
        self.ext_id_to_slab_key.insert(ext_id, slab_key);
        slab_key.into()
    }
    pub(crate) fn int_id_by_ext_id(&self, ext_id: &ExtId) -> Option<IntId> {
        self.ext_id_to_slab_key.get(ext_id).map(|&slab_key| slab_key.into())
    }
    pub(crate) fn int_id_by_ext_id_err(&self, ext_id: &ExtId) -> Result<IntId, Err> {
        match self.ext_id_to_slab_key.get(ext_id) {
            Some(&slab_key) => Ok(slab_key.into()),
            None => Err(Err::from(*ext_id)),
        }
    }
    pub(crate) fn ext_id_by_int_id(&self, int_id: IntId) -> ExtId {
        self.get(int_id).get_id()
    }
    pub(crate) fn try_get(&self, int_id: IntId) -> Option<&T> {
        self.data.get(int_id.into())
    }
    pub(crate) fn get(&self, int_id: IntId) -> &T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(int_id.into()).unwrap()
    }
    pub(crate) fn get_mut(&mut self, int_id: IntId) -> &mut T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(int_id.into()).unwrap()
    }
    pub(crate) fn remove(&mut self, int_id: IntId) -> T {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(int_id.into());
        self.ext_id_to_slab_key.remove(&entity.get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (IntId, &T)> {
        self.ext_id_to_slab_key
            .values()
            .map(|&slab_key| (slab_key.into(), self.data.get(slab_key).unwrap()))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (IntId, &mut T)> {
        self.data.iter_mut().map(|(slab_key, entity)| (slab_key.into(), entity))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = IntId> {
        self.ext_id_to_slab_key.values().map(|&slab_key| slab_key.into())
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.ext_id_to_slab_key
            .values()
            .map(|&slab_key| self.data.get(slab_key).unwrap())
    }
    pub(crate) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, entity)| entity)
    }
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}
impl<T, ExtId, IntId, Err> Clone for UEntityContainer<T, ExtId, IntId, Err>
where
    T: Clone,
    ExtId: Copy,
{
    fn clone(&self) -> Self {
        Self {
            counter: self.counter,
            data: self.data.clone(),
            ext_id_to_slab_key: self.ext_id_to_slab_key.clone(),
            phantom_int: PhantomData,
            phantom_error: PhantomData,
        }
    }
}
