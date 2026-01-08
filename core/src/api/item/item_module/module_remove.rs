use super::shared::get_fit_rack_mut;
use crate::{
    api::{ModuleMut, RmMode},
    misc::Index,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module(
        &mut self,
        module_uid: UItemId,
        pos_mode: RmMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let fit_uid = u_module.get_fit_uid();
        let rack = u_module.get_rack();
        let charge_uid = u_module.get_charge_uid();
        // Remove outgoing projections for both module and charge
        if !u_module.get_projs().is_empty() {
            // Remove outgoing projections for module
            for projectee_uid in u_module.get_projs().iter_projectees() {
                // Remove module outgoing projections from services
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, module_uid, projectee_uid);
                // Remove module outgoing projections from reverse projection tracker
                self.rev_projs.unreg_projectee(&module_uid, projectee_uid);
            }
            let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
            u_module.get_projs_mut().clear();
            // Remove outgoing projections for charge
            if let Some(charge_uid) = charge_uid {
                let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
                for projectee_uid in u_charge.get_projs().iter_projectees() {
                    // Remove charge outgoing projections from services
                    SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_uid, projectee_uid);
                    // Remove charge outgoing projections from reverse projection tracker
                    self.rev_projs.unreg_projectee(&charge_uid, projectee_uid);
                }
                let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
                u_charge.get_projs_mut().clear();
            }
        }
        // Remove charge from services
        if let Some(charge_uid) = charge_uid {
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        }
        // Remove module from services
        SolarSystem::util_remove_module(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        // Update user data - not updating module<->charge references because both will be removed
        if let Some(charge_uid) = charge_uid {
            self.u_data.items.remove(charge_uid);
        }
        let u_fit_rack = get_fit_rack_mut(&mut self.u_data.fits, fit_uid, rack);
        match pos_mode {
            RmMode::Free => u_fit_rack.free(&module_uid),
            RmMode::Remove => {
                if let Some(pos) = u_fit_rack.remove(&module_uid) {
                    for (i, rack_module_uid) in u_fit_rack.inner()[pos..].iter().enumerate() {
                        if let Some(rack_module_uid) = rack_module_uid {
                            self.u_data
                                .items
                                .get_mut(*rack_module_uid)
                                .dc_module_mut()
                                .unwrap()
                                .set_pos(Index::from_usize(pos + i));
                        }
                    }
                }
            }
        }
        self.u_data.items.remove(module_uid);
    }
}

impl<'a> ModuleMut<'a> {
    pub fn remove(self, pos_mode: RmMode) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_module(self.uid, pos_mode, &mut reuse_eupdates)
    }
}
