use std::{hash::Hash, marker::PhantomData};

use slab::Slab;

use crate::util::{ArenaId, LibDefault, LibGetId, LibIncrement, LibNamed, RMap};

pub(crate) struct UEntityContainer<T, ExtId, IntId, Err> {
    counter: ExtId,
    pub(super) data: Slab<T>,
    pub(super) ext_id_to_index: RMap<ExtId, usize>,
    phantom: PhantomData<(IntId, Err)>,
}
impl<T, ExtId, IntId, Err> UEntityContainer<T, ExtId, IntId, Err>
where
    T: LibGetId<ExtId> + LibNamed,
    IntId: ArenaId,
    ExtId: Copy + Eq + Hash + LibDefault + LibIncrement,
    Err: From<ExtId>,
{
    pub(in crate::ud) fn new(capacity: usize) -> Self {
        Self {
            counter: ExtId::lib_default(),
            data: Slab::with_capacity(capacity),
            ext_id_to_index: RMap::with_capacity(capacity),
            phantom: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> ExtId {
        let start = self.counter;
        while self.ext_id_to_index.contains_key(&self.counter) {
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
        let index = self.data.insert(entity);
        self.ext_id_to_index.insert(ext_id, index);
        IntId::new(index)
    }
    pub(crate) fn int_id_by_ext_id(&self, ext_id: &ExtId) -> Option<IntId> {
        self.ext_id_to_index.get(ext_id).map(|&index| IntId::new(index))
    }
    pub(crate) fn int_id_by_ext_id_err(&self, ext_id: &ExtId) -> Result<IntId, Err> {
        match self.ext_id_to_index.get(ext_id) {
            Some(&index) => Ok(IntId::new(index)),
            None => Err(Err::from(*ext_id)),
        }
    }
    pub(crate) fn ext_id_by_int_id(&self, int_id: IntId) -> ExtId {
        self.get(int_id).lib_get_id()
    }
    pub(crate) fn try_get(&self, int_id: IntId) -> Option<&T> {
        self.data.get(int_id.index())
    }
    pub(crate) fn get(&self, int_id: IntId) -> &T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(int_id.index()).unwrap()
    }
    pub(crate) fn get_mut(&mut self, int_id: IntId) -> &mut T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(int_id.index()).unwrap()
    }
    pub(crate) fn remove(&mut self, int_id: IntId) -> T {
        // Internal IDs are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(int_id.index());
        self.ext_id_to_index.remove(&entity.lib_get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (IntId, &T)> {
        self.ext_id_to_index
            .values()
            .map(|&index| (IntId::new(index), self.data.get(index).unwrap()))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = IntId> {
        self.ext_id_to_index.values().map(|&index| IntId::new(index))
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.ext_id_to_index
            .values()
            .map(|&index| self.data.get(index).unwrap())
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
            ext_id_to_index: self.ext_id_to_index.clone(),
            phantom: PhantomData,
        }
    }
}
