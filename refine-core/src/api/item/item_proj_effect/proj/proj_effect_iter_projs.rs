use crate::{Proj, ProjEffect, ProjEffectMut, ProjIter, api::iter_projs};

impl<'s> ProjEffect<'s> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.uid)
    }
}

impl<'s> ProjEffectMut<'s> {
    /// Iterates over projected effect's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = Proj<'_>> {
        iter_projs(self.sol, self.uid)
    }
    /// Iterates over projected effect's projections.
    pub fn iter_projs_mut(&mut self) -> ProjIter<'_> {
        ProjIter::new(self.sol, self.uid)
    }
}
