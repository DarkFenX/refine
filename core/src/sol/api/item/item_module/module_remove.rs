use super::shared::get_fit_rack_mut;
use crate::sol::{ItemKey, RmMode, SolarSystem, api::ModuleMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module(&mut self, item_key: ItemKey, pos_mode: RmMode) {
        let uad_item = self.uad.items.get(item_key);
        let uad_module = uad_item.get_module().unwrap();
        let fit_key = uad_module.get_fit_key();
        let rack = uad_module.get_rack();
        let charge_key = uad_module.get_charge_item_key();
        // Remove outgoing projections for both module and charge
        if let Some(charge_key) = charge_key {
            let charge_uad_item = self.uad.items.get(charge_key);
            // Use module projections, since module and charge projections should always match
            for &projectee_item_key in uad_module.get_projs().iter_projectee_item_keys() {
                let projectee_uad_item = self.uad.items.get(projectee_item_key);
                // Remove charge outgoing projections from services
                SolarSystem::util_remove_item_projection(
                    &self.uad,
                    &mut self.svc,
                    &self.reffs,
                    charge_key,
                    charge_uad_item,
                    projectee_item_key,
                    projectee_uad_item,
                );
                // Remove charge outgoing projections from reverse projection tracker
                self.rprojs.unreg_projectee(&charge_key, &projectee_item_key);
            }
        }
        for &projectee_item_key in uad_module.get_projs().iter_projectee_item_keys() {
            // Remove module outgoing projections from services
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                item_key,
                uad_item,
                projectee_item_key,
                projectee_uad_item,
            );
            // Remove module outgoing projections from reverse projection tracker
            self.rprojs.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove charge from services
        if let Some(charge_key) = charge_key {
            let charge_uad_item = self.uad.items.get(charge_key);
            SolarSystem::util_remove_item_without_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                charge_key,
                charge_uad_item,
            );
        }
        // Remove module from services
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Update user data - not updating module<->charge references because both will be removed
        if let Some(charge_key) = charge_key {
            self.uad.items.remove(charge_key);
        }
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
