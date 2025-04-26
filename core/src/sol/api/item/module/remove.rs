use itertools::Itertools;

use crate::sol::{ItemKey, RmMode, SolarSystem, api::ModuleMut};

use super::shared::get_fit_rack_mut;

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_module(&mut self, item_key: ItemKey, pos_mode: RmMode) {
        let uad_item = self.uad.items.get(item_key);
        let uad_module = uad_item.get_module().unwrap();
        let fit_key = uad_module.get_fit_key();
        let rack = uad_module.get_rack();
        let charge_key = uad_module.get_charge_item_key();
        // Remove outgoing projections for both module and charge
        let module_projectee_item_keys = uad_module.get_projs().iter_projectee_item_keys().copied().collect_vec();
        if !module_projectee_item_keys.is_empty() {
            if let Some(charge_key) = charge_key {
                // Use module projections, since module and charge projections should always match
                for &projectee_item_key in module_projectee_item_keys.iter() {
                    let projectee_uad_item = self.uad.items.get(projectee_item_key);
                    // Update services for charge
                    self.svc
                        .remove_item_projection(&self.uad, charge_key, projectee_item_key, projectee_uad_item);
                    // Update user data for charge - don't touch data on charge itself, since charge
                    // will be removed later anyway
                    self.proj_tracker.unreg_projectee(&charge_key, &projectee_item_key);
                }
            }
            for projectee_item_id in module_projectee_item_keys {
                // Update services for module
                let projectee_uad_item = self.uad.items.get(projectee_item_id);
                self.svc
                    .remove_item_projection(&self.uad, item_key, projectee_item_id, projectee_uad_item);
                // Update user data for module - don't touch data on module itself, since module
                // will be removed later anyway
                self.proj_tracker.unreg_projectee(&item_key, &projectee_item_id);
            }
        }
        // Remove charge
        if let Some(charge_key) = charge_key {
            // Update services for charge
            let charge_uad_item = self.uad.items.get(charge_key);
            self.svc.remove_item(&self.uad, charge_key, charge_uad_item);
            // Update user data for charge - not updating module<->charge references because both
            // will be removed
            self.uad.items.remove(charge_key);
        }
        // Remove module
        // Update services for module
        self.internal_remove_item_key_from_svc(item_key);
        // Update user data for module
        let uad_fit_rack = get_fit_rack_mut(&mut self.uad.fits, fit_key, rack);
        match pos_mode {
            RmMode::Free => uad_fit_rack.free(&item_key),
            RmMode::Remove => {
                if let Some(pos) = uad_fit_rack.remove(&item_key) {
                    for (i, rack_module_key) in uad_fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.uad
                                .items
                                .get_mut(*rack_module_key)
                                .get_module_mut()
                                .unwrap()
                                .set_pos(pos + i);
                        }
                    }
                }
            }
        }
        self.uad.items.remove(item_key);
    }
}

impl<'a> ModuleMut<'a> {
    pub fn remove(self, pos_mode: RmMode) {
        self.sol.internal_remove_module(self.key, pos_mode);
    }
}
