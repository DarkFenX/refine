use crate::api::{Fighter, FighterMut, RangedProj, RangedProjIter, iter_ranged_projs};

impl<'a> Fighter<'a> {
    /// Iterates over fighter's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
}

impl<'a> FighterMut<'a> {
    /// Iterates over fighter's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
    /// Iterates over fighter's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.key)
    }
}
