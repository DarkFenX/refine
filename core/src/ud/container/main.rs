use std::{hash::Hash, marker::PhantomData};

use slab::Slab;

use crate::util::{LibDefault, LibGetId, LibIncrement, LibNamed, RMap};

pub(crate) struct UEntityContainer<T, ExtId, IntId, Err>
where
    T: LibGetId<ExtId> + LibNamed,
    IntId: Copy + From<usize> + Into<usize>,
    ExtId: Copy + Eq + Hash + LibDefault + LibIncrement,
    Err: From<ExtId>,
{
    counter: ExtId,
    pub(super) data: Slab<T>,
    pub(super) eid_to_key: RMap<ExtId, usize>,
    phantom_iid: PhantomData<IntId>,
    phantom_error: PhantomData<Err>,
}
impl<T, ExtId, IntId, Err> UEntityContainer<T, ExtId, IntId, Err>
where
    T: LibGetId<ExtId> + LibNamed,
    IntId: Copy + From<usize> + Into<usize>,
    ExtId: Copy + Eq + Hash + LibDefault + LibIncrement,
    Err: From<ExtId>,
{
    pub(in crate::ud) fn new(capacity: usize) -> Self {
        Self {
            counter: ExtId::lib_default(),
            data: Slab::with_capacity(capacity),
            eid_to_key: RMap::with_capacity(capacity),
            phantom_iid: PhantomData,
            phantom_error: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> ExtId {
        let start = self.counter;
        while self.eid_to_key.contains_key(&self.counter) {
            self.counter.lib_increment();
            if start == self.counter {
                panic!("ran out of {} ID space", T::lib_get_name());
            }
        }
        let eid = self.counter;
        self.counter.lib_increment();
        eid
    }
    pub(crate) fn add(&mut self, entity: T) -> IntId {
        let eid = entity.lib_get_id();
        let key = self.data.insert(entity);
        self.eid_to_key.insert(eid, key);
        key.into()
    }
    pub(crate) fn iid_by_eid(&self, eid: &ExtId) -> Option<IntId> {
        self.eid_to_key.get(eid).map(|&key| key.into())
    }
    pub(crate) fn iid_by_eid_err(&self, eid: &ExtId) -> Result<IntId, Err> {
        match self.eid_to_key.get(eid) {
            Some(&key) => Ok(key.into()),
            None => Err(Err::from(*eid)),
        }
    }
    pub(crate) fn eid_by_iid(&self, iid: IntId) -> ExtId {
        self.get(iid).lib_get_id()
    }
    pub(crate) fn try_get(&self, iid: IntId) -> Option<&T> {
        self.data.get(iid.into())
    }
    pub(crate) fn get(&self, iid: IntId) -> &T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(iid.into()).unwrap()
    }
    pub(crate) fn get_mut(&mut self, iid: IntId) -> &mut T {
        // Internal IDs are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(iid.into()).unwrap()
    }
    pub(crate) fn remove(&mut self, iid: IntId) -> T {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let entity = self.data.remove(iid.into());
        self.eid_to_key.remove(&entity.lib_get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (IntId, &T)> {
        self.eid_to_key
            .values()
            .map(|&key| (key.into(), self.data.get(key).unwrap()))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (IntId, &mut T)> {
        self.data.iter_mut().map(|(key, entity)| (key.into(), entity))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = IntId> {
        self.eid_to_key.values().map(|&key| key.into())
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.eid_to_key.values().map(|&key| self.data.get(key).unwrap())
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
            eid_to_key: self.eid_to_key.clone(),
            phantom_iid: PhantomData,
            phantom_error: PhantomData,
        }
    }
}
