use crate::sol::api::{Drone, DroneMut, RangedProj, RangedProjIter, iter_projectee_item_keys, iter_ranged_projs};

impl<'a> Drone<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_ranged_projs(self.sol, self.key)
    }
}

impl<'a> DroneMut<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_ranged_projs(self.sol, self.key)
    }
    /// Iterates over drone's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter {
        let projectee_keys = iter_projectee_item_keys(self.sol, self.key).collect();
        RangedProjIter::new(self.sol, self.key, projectee_keys)
    }
}
