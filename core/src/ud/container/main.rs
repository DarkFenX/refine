use std::{hash::Hash, marker::PhantomData};

use slab::Slab;

use crate::util::{LibDefault, LibGetId, LibIncrement, LibNamed, RMap};

pub(crate) struct UEntityContainer<T, ExtId, IntId, Err> {
    counter: ExtId,
    pub(super) data: Slab<T>,
    pub(super) xid_to_slab_key: RMap<ExtId, usize>,
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
            xid_to_slab_key: RMap::with_capacity(capacity),
            phantom_iid: PhantomData,
            phantom_error: PhantomData,
        }
    }
    pub(crate) fn alloc_id(&mut self) -> ExtId {
        let start = self.counter;
        while self.xid_to_slab_key.contains_key(&self.counter) {
            self.counter.lib_increment();
            if start == self.counter {
                panic!("ran out of {} ID space", T::lib_get_name());
            }
        }
        let xid = self.counter;
        self.counter.lib_increment();
        xid
    }
    pub(crate) fn add(&mut self, entity: T) -> IntId {
        let xid = entity.lib_get_id();
        let slab_key = self.data.insert(entity);
        self.xid_to_slab_key.insert(xid, slab_key);
        slab_key.into()
    }
    pub(crate) fn iid_by_xid(&self, xid: &ExtId) -> Option<IntId> {
        self.xid_to_slab_key.get(xid).map(|&key| key.into())
    }
    pub(crate) fn iid_by_xid_err(&self, xid: &ExtId) -> Result<IntId, Err> {
        match self.xid_to_slab_key.get(xid) {
            Some(&slab_key) => Ok(slab_key.into()),
            None => Err(Err::from(*xid)),
        }
    }
    pub(crate) fn xid_by_iid(&self, iid: IntId) -> ExtId {
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
        self.xid_to_slab_key.remove(&entity.lib_get_id());
        entity
    }
    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (IntId, &T)> {
        self.xid_to_slab_key
            .values()
            .map(|&slab_key| (slab_key.into(), self.data.get(slab_key).unwrap()))
    }
    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (IntId, &mut T)> {
        self.data.iter_mut().map(|(slab_key, entity)| (slab_key.into(), entity))
    }
    pub(crate) fn keys(&self) -> impl ExactSizeIterator<Item = IntId> {
        self.xid_to_slab_key.values().map(|&key| key.into())
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.xid_to_slab_key.values().map(|&key| self.data.get(key).unwrap())
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
            xid_to_slab_key: self.xid_to_slab_key.clone(),
            phantom_iid: PhantomData,
            phantom_error: PhantomData,
        }
    }
}
