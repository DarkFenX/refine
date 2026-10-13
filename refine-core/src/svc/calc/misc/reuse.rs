use crate::{
    svc::calc::{CtxModifier, RawModId, RawModifier},
    ud::UItemId,
};

#[derive(Clone, Default)]
pub(in crate::svc::calc) struct CalcReuse {
    pub(in crate::svc::calc) rmod_ids: ReuseVec<RawModId>,
    pub(in crate::svc::calc) rmods: ReuseVec<RawModifier>,
    pub(in crate::svc::calc) items: ReuseVec<UItemId>,
    pub(in crate::svc::calc) cmods: ReuseVec<CtxModifier>,
}

#[derive(Clone)]
pub(in crate::svc::calc) struct ReuseVec<T> {
    vec: Vec<T>,
}
impl<T> Default for ReuseVec<T> {
    fn default() -> Self {
        Self { vec: Vec::new() }
    }
}
impl<T> ReuseVec<T> {
    pub(in crate::svc::calc) fn take(&mut self) -> Vec<T> {
        std::mem::take(&mut self.vec)
    }
    pub(in crate::svc::calc) fn put(&mut self, mut vec: Vec<T>) {
        // Nested calls might have put their container back already, keep the bigger one
        if vec.capacity() > self.vec.capacity() {
            vec.clear();
            self.vec = vec;
        }
    }
}
