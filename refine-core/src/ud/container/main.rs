use std::{hash::Hash, marker::PhantomData};

use crate::util::{LibDefault, LibGetId, LibIncrement, LibNamed, PSlab, RMap, SlabId};

pub(crate) struct UEntityContainer<T, ExtId, IntId, Err> {
    counter: ExtId,
    pub(super) data: PSlab<IntId, T>,
    pub(super) ext_id_to_int_id: RMap<ExtId, IntId>,
    phantom: PhantomData<Err>,
}
impl<T, ExtId, IntId, Err> UEntityContainer<T, ExtId, IntId, Err>
where
    T: LibGetId<ExtId> + LibNamed,
    IntId: SlabId,
    ExtId: Copy + Eq + Hash + LibDefault + LibIncrement,
    Err: From<ExtId>,
{
    pub(in crate::ud) fn new(capacity: usize) -> Self {
        Self {
            counter: ExtId::lib_default(),
            data: PSlab::with_capacity(capacity),
            ext_id_to_int_id: RMap::with_capacity(capacity),
            phantom: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> ExtId {
        let start = self.counter;
        while self.ext_id_to_int_id.contains_key(&self.counter) {
            self.counter.lib_increment();
            if start == self.counter {
                panic!("ran out of {} ID space", T::lib_get_name());
            }
        }
        let ext_id = self.counter;
        self.counter.lib_increment();
        ext_id
    }
    pub(crate) fn add(&mut self, entity: T) -> IntId {
        let ext_id = entity.lib_get_id();
        let int_id = self.data.insert(entity);
        self.ext_id_to_int_id.insert(ext_id, int_id);
        int_id
    }
    pub(crate) fn int_id_by_ext_id(&self, ext_id: &ExtId) -> Option<IntId> {
        self.ext_id_to_int_id.get(ext_id).copied()
    }
    pub(crate) fn int_id_by_ext_id_err(&self, ext_id: &ExtId) -> Result<IntId, Err> {
        match self.ext_id_to_int_id.get(ext_id) {
            Some(&int_id) => Ok(int_id),
            None => Err(Err::from(*ext_id)),
        }
    }
    pub(crate) fn ext_id_by_int_id(&self, int_id: IntId) -> ExtId {
        self.get(int_id).lib_get_id()
    }
    pub(crate) fn try_get(&self, int_id: IntId) -> Option<&T> {
        self.data.get(int_id)
    }
    pub(crate) fn get(&self, int_id: IntId) -> &T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(int_id).unwrap()
    }
    pub(crate) fn get_mut(&mut self, int_id: IntId) -> &mut T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(int_id).unwrap()
    }
    pub(crate) fn remove(&mut self, int_id: IntId) -> T {
        // Internal IDs are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(int_id);
        self.ext_id_to_int_id.remove(&entity.lib_get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (IntId, &T)> {
        self.ext_id_to_int_id
            .values()
            .map(|&int_id| (int_id, self.data.get(int_id).unwrap()))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = IntId> {
        self.ext_id_to_int_id.values().copied()
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.ext_id_to_int_id
            .values()
            .map(|&int_id| self.data.get(int_id).unwrap())
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
    IntId: Copy,
{
    fn clone(&self) -> Self {
        Self {
            counter: self.counter,
            data: self.data.clone(),
            ext_id_to_int_id: self.ext_id_to_int_id.clone(),
            phantom: PhantomData,
        }
    }
}
