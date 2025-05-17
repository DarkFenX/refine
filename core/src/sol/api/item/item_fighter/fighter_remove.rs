use itertools::Itertools;

use crate::sol::{ItemKey, SolarSystem, api::FighterMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter(&mut self, item_key: ItemKey) {
        SolarSystem::remove_fighter_autocharges(&mut self.svc, &mut self.uad, &mut self.proj_tracker, item_key);
        // Remove outgoing projections
        let uad_item = self.uad.items.get(item_key);
        let fit_key = uad_item.get_fighter().unwrap().get_fit_key();
        let projectee_item_keys = uad_item
            .get_fighter()
            .unwrap()
            .get_projs()
            .iter_projectee_item_keys()
            .copied()
            .collect_vec();
        if !projectee_item_keys.is_empty() {
            for projectee_item_key in projectee_item_keys.into_iter() {
                let projectee_uad_item = self.uad.items.get(projectee_item_key);
                self.svc
                    .remove_item_projection(&self.uad, item_key, uad_item, projectee_item_key, projectee_uad_item);
                self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
            }
            // Clear on-fighter projections, so that they don't get processed 2nd time on fighter
            // removal from services
            self.uad
                .items
                .get_mut(item_key)
                .get_fighter_mut()
                .unwrap()
                .get_projs_mut()
                .clear();
        }
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Update services
        self.internal_remove_item_key_from_svc(item_key);
        // Update user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.fighters.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> FighterMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fighter(self.key);
    }
}
