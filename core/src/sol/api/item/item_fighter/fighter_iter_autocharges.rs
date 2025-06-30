use crate::{
    def::ItemKey,
    sol::{
        SolarSystem,
        api::{Autocharge, AutochargeMut, Fighter, FighterMut, MutIter},
    },
};

impl<'a> Fighter<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge<'_>> {
        iter_autocharges(self.sol, self.key)
    }
}

impl<'a> FighterMut<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge<'_>> {
        iter_autocharges(self.sol, self.key)
    }
    pub fn iter_autocharges_mut(&mut self) -> MutIter<'_, AutochargeMut<'_>> {
        let autocharge_keys = self
            .sol
            .uad
            .items
            .get(self.key)
            .get_fighter()
            .unwrap()
            .get_autocharges()
            .values()
            .copied()
            .collect();
        MutIter::new(self.sol, autocharge_keys)
    }
}

fn iter_autocharges(sol: &SolarSystem, fighter_key: ItemKey) -> impl Iterator<Item = Autocharge<'_>> {
    sol.uad
        .items
        .get(fighter_key)
        .get_fighter()
        .unwrap()
        .get_autocharges()
        .values()
        .map(|&autocharge_key| Autocharge::new(sol, autocharge_key))
}
