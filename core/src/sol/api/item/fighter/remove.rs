use itertools::Itertools;

use crate::sol::{ItemKey, SolarSystem, api::FighterMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_fighter(&mut self, item_key: ItemKey) {
        // Check if everything is correct and collect autocharge IDs to be used later
        let uad_item = self.uad.items.get(item_key);
        let uad_fighter = uad_item.get_fighter().unwrap();
        let fit_key = uad_fighter.get_fit_key();
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Remove outgoing projections for fighter and its autocharges
        for &projectee_item_key in uad_fighter.get_projs().iter_projectee_item_keys() {
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            for &autocharge_key in autocharge_keys.iter() {
                // Update services for autocharge
                self.svc
                    .remove_item_projection(&self.uad, autocharge_key, projectee_item_key, projectee_uad_item);
                // Update user data for autocharge - don't touch data on charge itself, since charge
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            }
            // Update services for fighter
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
            // Update user data for fighter - don't touch data on fighter itself, since fighter will
            // be removed later anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_key);
        // Remove autocharges
        for autocharge_key in autocharge_keys {
            // Update services for autocharge
            self.remove_item_key_from_svc(autocharge_key);
            // Update user data for autocharge - not updating fighter<->autocharge references
            // because both will be removed
            self.uad.items.remove(autocharge_key);
        }
        // Remove fighter
        // Update services for fighter
        self.remove_item_key_from_svc(item_key);
        // Update user data for fighter
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
