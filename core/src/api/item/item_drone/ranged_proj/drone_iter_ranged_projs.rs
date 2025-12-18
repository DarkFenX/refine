use crate::api::{Drone, DroneMut, RangedProj, RangedProjIter, iter_ranged_projs};

impl<'a> Drone<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
}

impl<'a> DroneMut<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.key)
    }
    /// Iterates over drone's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.key)
    }
}
