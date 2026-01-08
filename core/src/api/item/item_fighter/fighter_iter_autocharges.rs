use crate::{
    api::{Autocharge, AutochargeMut, Fighter, FighterMut, MutIter},
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Fighter<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge<'_>> {
        iter_autocharges(self.sol, self.uid)
    }
}

impl<'a> FighterMut<'a> {
    pub fn iter_autocharges(&self) -> impl Iterator<Item = Autocharge<'_>> {
        iter_autocharges(self.sol, self.uid)
    }
    pub fn iter_autocharges_mut(&mut self) -> MutIter<'_, AutochargeMut<'_>> {
        let u_fighter = self.sol.u_data.items.get(self.uid).dc_fighter().unwrap();
        let autocharge_uids = u_fighter.get_autocharges().values().collect();
        MutIter::new(self.sol, autocharge_uids)
    }
}

fn iter_autocharges(sol: &SolarSystem, fighter_uid: UItemId) -> impl Iterator<Item = Autocharge<'_>> {
    let u_fighter = sol.u_data.items.get(fighter_uid).dc_fighter().unwrap();
    let autocharge_uids = u_fighter.get_autocharges().values();
    autocharge_uids.map(|autocharge_uid| Autocharge::new(sol, autocharge_uid))
}
