use itertools::Itertools;

use crate::sol::{ItemKey, SolarSystem, api::ChargeMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_charge(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_charge = uad_item.get_charge().unwrap();
        let module_item_key = uad_charge.get_cont_item_key();
        let projectee_item_keys = uad_charge.get_projs().iter_projectee_item_keys().copied().collect_vec();
        // Remove outgoing projections
        if !projectee_item_keys.is_empty() {
            for projectee_item_key in projectee_item_keys.into_iter() {
                // Update services
                let projectee_uad_item = self.uad.items.get(projectee_item_key);
                self.svc
                    .remove_item_projection(&self.uad, item_key, uad_item, projectee_item_key, projectee_uad_item);
                // Projection tracker
                self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
            }
            // Clear on-charge projections, so that they don't get processed 2nd time on charge
            // removal from services
            self.uad
                .items
                .get_mut(item_key)
                .get_charge_mut()
                .unwrap()
                .get_projs_mut()
                .clear();
        }
        // Update services
        self.internal_remove_item_key_from_svc(item_key);
        // Update user data
        let uad_module = self.uad.items.get_mut(module_item_key).get_module_mut().unwrap();
        uad_module.set_charge_item_key(None);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_charge(self.key);
    }
}
