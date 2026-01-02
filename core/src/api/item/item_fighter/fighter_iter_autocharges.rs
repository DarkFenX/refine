use crate::{
    api::{Autocharge, AutochargeMut, Fighter, FighterMut, MutIter},
    sol::SolarSystem,
    ud::UItemId,
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
        let u_fighter = self.sol.u_data.items.get(self.key).dc_fighter().unwrap();
        let autocharge_keys = u_fighter.get_autocharges().values().collect();
        MutIter::new(self.sol, autocharge_keys)
    }
}

fn iter_autocharges(sol: &SolarSystem, fighter_key: UItemId) -> impl Iterator<Item = Autocharge<'_>> {
    let u_fighter = sol.u_data.items.get(fighter_key).dc_fighter().unwrap();
    let autocharge_keys = u_fighter.get_autocharges().values();
    autocharge_keys.map(|autocharge_key| Autocharge::new(sol, autocharge_key))
}
