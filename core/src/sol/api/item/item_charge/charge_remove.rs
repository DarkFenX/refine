use crate::{
    sol::{SolarSystem, api::ChargeMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_charge(
        &mut self,
        charge_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_charge = self.u_data.items.get(charge_key).get_charge().unwrap();
        let module_key = u_charge.get_cont_item_key();
        // Remove outgoing projections
        if !u_charge.get_projs().is_empty() {
            for projectee_key in u_charge.get_projs().iter_projectees() {
                // Update services
                SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_key, projectee_key);
                // Reverse projections
                self.rev_projs.unreg_projectee(&charge_key, &projectee_key);
            }
            let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
            u_charge.get_projs_mut().clear();
        }
        // Update services
        SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_key, reuse_eupdates);
        // Update user data
        let u_module = self.u_data.items.get_mut(module_key).get_module_mut().unwrap();
        u_module.set_charge_key(None);
        self.u_data.items.remove(charge_key);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_charge(self.key, &mut reuse_eupdates)
    }
}
