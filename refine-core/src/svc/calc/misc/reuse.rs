use crate::{
    svc::calc::{CtxModKey, CtxModifier, RawModId, RawModifier},
    ud::UItemId,
};

#[derive(Clone, Default)]
pub(in crate::svc::calc) struct CalcReuse {
    pub(in crate::svc::calc) items: ReuseVec<UItemId>,
    pub(in crate::svc::calc) rmod_ids: ReuseVec<RawModId>,
    pub(in crate::svc::calc) rmods: ReuseVec<RawModifier>,
    pub(in crate::svc::calc) ckeys: ReuseVec<CtxModKey>,
    pub(in crate::svc::calc) cmods: ReuseVec<CtxModifier>,
}

const MAX_SPARES: usize = 5;

pub(in crate::svc::calc) struct ReuseVec<T> {
    primary: Vec<T>,
    spares: Vec<Vec<T>>,
}
impl<T> Default for ReuseVec<T> {
    fn default() -> Self {
        Self {
            primary: Vec::new(),
            spares: Vec::new(),
        }
    }
}
impl<T> Clone for ReuseVec<T> {
    fn clone(&self) -> Self {
        // Spare containers are just a cache, no need to copy them
        Self::default()
    }
}
impl<T> ReuseVec<T> {
    pub(in crate::svc::calc) fn take(&mut self) -> Vec<T> {
        match self.primary.capacity() {
            0 => self.spares.pop().unwrap_or_default(),
            _ => std::mem::take(&mut self.primary),
        }
    }
    pub(in crate::svc::calc) fn put(&mut self, mut vec: Vec<T>) {
        if vec.capacity() == 0 {
            return;
        }
        vec.clear();
        if self.primary.capacity() == 0 {
            self.primary = vec;
            return;
        }
        if self.spares.len() < MAX_SPARES {
            self.spares.push(vec);
        }
    }
}
