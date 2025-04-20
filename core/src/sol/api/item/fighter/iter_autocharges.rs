use crate::sol::{
    ItemKey, SolarSystem,
    api::{Autocharge, AutochargeMutGenerator, Fighter, FighterMut, ItemMutIter},
};

impl<'a> Fighter<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge> {
        iter_autocharges(self.sol, self.key)
    }
}

impl<'a> FighterMut<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge> {
        iter_autocharges(self.sol, self.key)
    }
    pub fn iter_autocharges_mut(&mut self) -> ItemMutIter<'_, AutochargeMutGenerator> {
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
        ItemMutIter::new(self.sol, autocharge_keys)
    }
}

fn iter_autocharges(sol: &SolarSystem, fighter_key: ItemKey) -> impl Iterator<Item = Autocharge> {
    sol.uad
        .items
        .get(fighter_key)
        .get_fighter()
        .unwrap()
        .get_autocharges()
        .values()
        .map(|&autocharge_key| Autocharge::new(sol, autocharge_key))
}
