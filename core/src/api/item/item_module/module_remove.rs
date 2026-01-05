use super::shared::get_fit_rack_mut;
use crate::{
    api::{ModuleMut, RmMode},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module(
        &mut self,
        module_key: UItemId,
        pos_mode: RmMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_module = self.u_data.items.get(module_key).dc_module().unwrap();
        let fit_key = u_module.get_fit_uid();
        let rack = u_module.get_rack();
        let charge_key = u_module.get_charge_uid();
        // Remove outgoing projections for both module and charge
        if !u_module.get_projs().is_empty() {
            // Remove outgoing projections for module
            for projectee_key in u_module.get_projs().iter_projectees() {
                // Remove module outgoing projections from services
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, module_key, projectee_key);
                // Remove module outgoing projections from reverse projection tracker
                self.rev_projs.unreg_projectee(&module_key, projectee_key);
            }
            let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
            u_module.get_projs_mut().clear();
            // Remove outgoing projections for charge
            if let Some(charge_key) = charge_key {
                let u_charge = self.u_data.items.get(charge_key).dc_charge().unwrap();
                for projectee_key in u_charge.get_projs().iter_projectees() {
                    // Remove charge outgoing projections from services
                    SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_key, projectee_key);
                    // Remove charge outgoing projections from reverse projection tracker
                    self.rev_projs.unreg_projectee(&charge_key, projectee_key);
                }
                let u_charge = self.u_data.items.get_mut(charge_key).dc_charge_mut().unwrap();
                u_charge.get_projs_mut().clear();
            }
        }
        // Remove charge from services
        if let Some(charge_key) = charge_key {
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_key, reuse_eupdates);
        }
        // Remove module from services
        SolarSystem::util_remove_module(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        // Update user data - not updating module<->charge references because both will be removed
        if let Some(charge_key) = charge_key {
            self.u_data.items.remove(charge_key);
        }
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_key, rack);
        match pos_mode {
            RmMode::Free => u_fit_rack.free(&module_key),
            RmMode::Remove => {
                if let Some(pos) = u_fit_rack.remove(&module_key) {
                    for (i, rack_module_key) in u_fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.u_data
                                .items
                                .get_mut(*rack_module_key)
                                .dc_module_mut()
                                .unwrap()
                                .set_pos(pos + i);
                        }
                    }
                }
            }
        }
        self.u_data.items.remove(module_key);
    }
}

impl<'a> ModuleMut<'a> {
    pub fn remove(self, pos_mode: RmMode) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_module(self.key, pos_mode, &mut reuse_eupdates)
    }
}
