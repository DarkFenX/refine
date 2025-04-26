use crate::sol::{
    ItemKey, SolarSystem,
    api::{Drone, DroneMut, RangedProj, RangedProjIter},
};

impl<'a> Drone<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_projs(self.sol, self.key)
    }
}

impl<'a> DroneMut<'a> {
    /// Iterates over drone's projections.
    pub fn iter_projs(&self) -> impl Iterator<Item = RangedProj> {
        iter_projs(self.sol, self.key)
    }
    /// Iterates over drone's projections.
    pub fn iter_projs_mut(&mut self) -> RangedProjIter {
        let projectee_keys = iter_projectee_item_keys(self.sol, self.key).collect();
        RangedProjIter::new(self.sol, self.key, projectee_keys)
    }
}

fn iter_projs(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = RangedProj> {
    iter_projectee_item_keys(sol, item_key)
        .map(move |projectee_item_key| RangedProj::new(sol, item_key, projectee_item_key))
}

fn iter_projectee_item_keys(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = ItemKey> + use<'_> {
    sol.uad.items.get(item_key).iter_projectee_item_keys().unwrap().copied()
}
