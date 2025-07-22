use crate::{
    sol::{SolarSystem, api::FighterMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        SolarSystem::remove_fighter_autocharges(
            &mut self.uad,
            &mut self.svc,
            &mut self.rprojs,
            item_key,
            false,
            reuse_eupdates,
        );
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let uad_fighter = uad_item.get_fighter().unwrap();
        let fit_key = uad_fighter.get_fit_key();
        for projectee_key in uad_fighter.get_projs().iter_projectees() {
            let projectee_uad_item = self.uad.items.get(projectee_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                item_key,
                uad_item,
                projectee_key,
                projectee_uad_item,
            );
            self.rprojs.unreg_projectee(&item_key, &projectee_key);
        }
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Update services
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        // Update user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.fighters.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> FighterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_fighter(self.key, &mut reuse_eupdates);
    }
}
