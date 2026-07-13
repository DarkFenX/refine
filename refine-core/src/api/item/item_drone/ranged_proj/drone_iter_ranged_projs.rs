use crate::api::{Drone, DroneMut, RangedProj, RangedProjIter, iter_ranged_projs};

impl<'s> Drone<'s> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.uid)
    }
}

impl<'s> DroneMut<'s> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl ExactSizeIterator<Item = RangedProj<'_>> {
        iter_ranged_projs(self.sol, self.uid)
    }
    /// Iterates over drone's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter<'_> {
        RangedProjIter::new(self.sol, self.uid)
    }
}
