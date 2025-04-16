use crate::sol::{
    ItemKey, SolarSystem,
    api::{Autocharge, Fighter, FighterMut},
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
