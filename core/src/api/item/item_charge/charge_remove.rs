use crate::{
    api::ChargeMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_charge(&mut self, charge_uid: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        let u_charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
        let module_uid = u_charge.get_cont_item_uid();
        // Remove outgoing projections
        if !u_charge.get_projs().is_empty() {
            for projectee_uid in u_charge.get_projs().iter_projectees() {
                // Update services
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_uid, projectee_uid);
                // Reverse projections
                self.rev_projs.unreg_projectee(&charge_uid, projectee_uid);
            }
            let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
            u_charge.get_projs_mut().clear();
        }
        // Update services
        SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        // Update user data
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_charge_uid(None);
        self.u_data.items.remove(charge_uid);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_charge(self.uid, &mut reuse_eupdates)
    }
}
