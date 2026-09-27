use crate::{
    api::{ModuleMut, ModuleState},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_state(&mut self, module_uid: UItemId, state: ModuleState) {
        // Update user data for module
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let charge_uid = u_module.get_charge_uid();
        let old_a_state = u_module.get_state();
        u_module.set_module_state(state);
        let new_a_state = u_module.get_state();
        u_module.update_reffs(&mut self.cache.eupdates, &self.u_data.r_data);
        // Update services for module
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            module_uid,
            old_a_state,
            new_a_state,
            &self.cache.eupdates,
        );
        if let Some(charge_activated) = self.cache.eupdates.charge
            && let Some(charge_uid) = charge_uid
        {
            SolarSystem::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_uid,
                charge_activated,
                &mut self.cache.eupdates,
            );
        }
    }
}

impl<'s> ModuleMut<'s> {
    pub fn set_state(&mut self, state: ModuleState) {
        self.sol.internal_set_module_state(self.uid, state)
    }
}
