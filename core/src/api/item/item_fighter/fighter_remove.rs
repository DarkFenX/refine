use crate::{
    api::FighterMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fighter(
        &mut self,
        fighter_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Remove incoming projections
        self.internal_remove_incoming_projections(fighter_key);
        // Remove autocharges with all the associated relations
        SolarSystem::remove_item_autocharges(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_key,
            reuse_eupdates,
        );
        // Remove outgoing projections
        let u_fighter = self.u_data.items.get(fighter_key).dc_fighter().unwrap();
        let fit_key = u_fighter.get_fit_uid();
        if !u_fighter.get_projs().is_empty() {
            for projectee_key in u_fighter.get_projs().iter_projectees() {
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, fighter_key, projectee_key);
                self.rev_projs.unreg_projectee(&fighter_key, projectee_key);
            }
            let u_fighter = self.u_data.items.get_mut(fighter_key).dc_fighter_mut().unwrap();
            u_fighter.get_projs_mut().clear();
        }
        // Update services
        SolarSystem::util_remove_fighter(&mut self.u_data, &mut self.svc, fighter_key, reuse_eupdates);
        // Update user data
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.fighters.remove(&fighter_key);
        self.u_data.items.remove(fighter_key);
    }
}

impl<'a> FighterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_fighter(self.key, &mut reuse_eupdates);
    }
}
