use std::hash::Hash;

use crate::{
    svc::calc::{CalcModification, CalcModificationKey, CtxModKey, CtxModifier, RawModId, RawModifier},
    ud::UItemId,
    util::RMap,
};

const MAX_SPARES: usize = 5;
// Clearing and iterating a map takes time proportional to its capacity, so maps which grew bigger
// than this are not kept, to avoid slowing down all the following uses
const MAX_MAP_CAPACITY: usize = 128;

#[derive(Clone, Default)]
pub(in crate::svc::calc) struct CalcReuse {
    pub(in crate::svc::calc) items: Reuse<Vec<UItemId>>,
    pub(in crate::svc::calc) rmod_ids: Reuse<Vec<RawModId>>,
    pub(in crate::svc::calc) rmods: Reuse<Vec<RawModifier>>,
    pub(in crate::svc::calc) ckeys: Reuse<Vec<CtxModKey>>,
    pub(in crate::svc::calc) cmods: Reuse<Vec<CtxModifier>>,
    pub(in crate::svc::calc) mods: Reuse<RMap<CalcModificationKey, CalcModification>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Trait for reusable container types
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::calc) trait Reusable: Default {
    fn is_reusable(&self) -> bool;
    fn clear(&mut self);
}
impl<T> Reusable for Vec<T> {
    fn is_reusable(&self) -> bool {
        self.capacity() > 0
    }
    fn clear(&mut self) {
        Vec::clear(self)
    }
}
impl<K, V> Reusable for RMap<K, V>
where
    K: Eq + Hash,
{
    fn is_reusable(&self) -> bool {
        let capacity = self.capacity();
        capacity > 0 && capacity <= MAX_MAP_CAPACITY
    }
    fn clear(&mut self) {
        RMap::clear(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reusable container storage
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::calc) struct Reuse<C> {
    primary: C,
    spares: Vec<C>,
}
impl<C> Default for Reuse<C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            primary: C::default(),
            spares: Vec::new(),
        }
    }
}
impl<C> Clone for Reuse<C>
where
    C: Default,
{
    fn clone(&self) -> Self {
        // Spare containers are just a cache, no need to copy them
        Self::default()
    }
}
impl<C> Reuse<C>
where
    C: Reusable,
{
    pub(in crate::svc::calc) fn take(&mut self) -> C {
        match self.primary.is_reusable() {
            true => std::mem::take(&mut self.primary),
            false => self.spares.pop().unwrap_or_default(),
        }
    }
    pub(in crate::svc::calc) fn put(&mut self, mut container: C) {
        if !container.is_reusable() {
            return;
        }
        container.clear();
        if !self.primary.is_reusable() {
            self.primary = container;
            return;
        }
        if self.spares.len() < MAX_SPARES {
            self.spares.push(container);
        }
    }
}
