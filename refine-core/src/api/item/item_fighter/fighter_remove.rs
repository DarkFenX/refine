use itertools::Itertools;

use crate::{
    api::FighterMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fighter(&mut self, fighter_uid: UItemId) {
        // Remove incoming projections
        self.internal_remove_incoming_projections(fighter_uid);
        // Remove autocharges from services; they are discarded along with the fighter
        SolarSystem::util_detach_item_autocharges(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
        // Remove outgoing projections; fighter keeps them, in case it gets reattached
        SolarSystem::util_unreg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, fighter_uid);
        // Update services
        SolarSystem::util_remove_fighter(&mut self.u_data, &mut self.svc, fighter_uid, &mut self.cache.eupdates);
        // Update user data
        let u_fighter = self.u_data.items.get(fighter_uid).dc_fighter().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_fighter.get_fit_uid());
        u_fit.fighters.remove(&fighter_uid);
        for autocharge_uid in u_fighter.get_autocharges().values().collect_vec() {
            self.internal_discard_child_item(autocharge_uid);
        }
        self.internal_discard_item(fighter_uid, UndoStep::FighterRemoved);
    }
}

impl<'s> FighterMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_fighter(self.uid);
    }
}
