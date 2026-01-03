use crate::{
    api::{ModuleMut, ModuleState},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_state(
        &mut self,
        module_key: UItemId,
        state: ModuleState,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Update user data for module
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        let charge_key = u_module.get_charge_uid();
        let old_a_state = u_module.get_state();
        u_module.set_module_state(state);
        let new_a_state = u_module.get_state();
        u_module.update_reffs(reuse_eupdates, &self.u_data.src);
        // Update services for module
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            module_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
        if let Some(charge_activated) = reuse_eupdates.charge
            && let Some(charge_key) = charge_key
        {
            SolarSystem::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_key,
                charge_activated,
                reuse_eupdates,
            );
        }
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_state(&mut self, state: ModuleState) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_module_state(self.key, state, &mut reuse_eupdates)
    }
}
