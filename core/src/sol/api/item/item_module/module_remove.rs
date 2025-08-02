use super::shared::get_fit_rack_mut;
use crate::{
    misc::RmMode,
    sol::{SolarSystem, api::ModuleMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module(
        &mut self,
        item_key: UItemKey,
        pos_mode: RmMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_module = self.u_data.items.get(item_key).get_module().unwrap();
        let fit_key = u_module.get_fit_key();
        let rack = u_module.get_rack();
        let charge_key = u_module.get_charge_key();
        // Remove outgoing projections for both module and charge
        if !u_module.get_projs().is_empty() {
            // Remove outgoing projections for module
            let u_item = self.u_data.items.get(item_key);
            for projectee_key in u_module.get_projs().iter_projectees() {
                // Remove module outgoing projections from services
                let projectee_u_item = self.u_data.items.get(projectee_key);
                SolarSystem::util_remove_item_projection(
                    &self.u_data,
                    &mut self.svc,
                    item_key,
                    u_item,
                    projectee_key,
                    projectee_u_item,
                );
                // Remove module outgoing projections from reverse projection tracker
                self.rev_projs.unreg_projectee(&item_key, &projectee_key);
            }
            let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
            u_module.get_projs_mut().clear();
            // Remove outgoing projections for charge
            if let Some(charge_key) = charge_key {
                let charge_u_item = self.u_data.items.get(charge_key);
                let u_charge = charge_u_item.get_charge().unwrap();
                for projectee_key in u_charge.get_projs().iter_projectees() {
                    let projectee_u_item = self.u_data.items.get(projectee_key);
                    // Remove charge outgoing projections from services
                    SolarSystem::util_remove_item_projection(
                        &self.u_data,
                        &mut self.svc,
                        charge_key,
                        charge_u_item,
                        projectee_key,
                        projectee_u_item,
                    );
                    // Remove charge outgoing projections from reverse projection tracker
                    self.rev_projs.unreg_projectee(&charge_key, &projectee_key);
                }
                let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
                u_charge.get_projs_mut().clear();
            }
        }
        // Remove charge from services
        if let Some(charge_key) = charge_key {
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_key, reuse_eupdates);
        }
        // Remove module from services
        SolarSystem::util_remove_module(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        // Update user data - not updating module<->charge references because both will be removed
        if let Some(charge_key) = charge_key {
            self.u_data.items.remove(charge_key);
        }
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_key, rack);
        match pos_mode {
            RmMode::Free => u_fit_rack.free(&item_key),
            RmMode::Remove => {
                if let Some(pos) = u_fit_rack.remove(&item_key) {
                    for (i, rack_module_key) in u_fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.u_data
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
        self.u_data.items.remove(item_key);
    }
}

impl<'a> ModuleMut<'a> {
    pub fn remove(self, pos_mode: RmMode) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_module(self.key, pos_mode, &mut reuse_eupdates)
    }
}
