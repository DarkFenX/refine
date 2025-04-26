use crate::sol::api::{Module, ModuleMut, RangedProj, RangedProjIter, iter_projectee_item_keys, iter_ranged_projs};

impl<'a> Module<'a> {
    /// Iterates over module's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_ranged_projs(self.sol, self.key)
    }
}

impl<'a> ModuleMut<'a> {
    /// Iterates over module's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_ranged_projs(self.sol, self.key)
    }
    /// Iterates over module's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter {
        let projectee_keys = iter_projectee_item_keys(self.sol, self.key).collect();
        RangedProjIter::new(self.sol, self.key, projectee_keys)
    }
}
