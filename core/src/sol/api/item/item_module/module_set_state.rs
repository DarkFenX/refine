use crate::{
    misc::ModuleState,
    sol::{SolarSystem, api::ModuleMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_state(
        &mut self,
        item_key: UItemKey,
        state: ModuleState,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Update user data for module
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = u_module.get_charge_key();
        let old_a_state = u_module.get_a_state();
        u_module.set_module_state(state, reuse_eupdates, &self.u_data.src);
        let new_a_state = u_module.get_a_state();
        // Update services for module
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
            let old_a_state = u_charge.get_a_state();
            u_charge.set_a_state(state.into(), reuse_eupdates, &self.u_data.src);
            let new_a_state = u_charge.get_a_state();
            // Update services for charge
            SolarSystem::util_switch_item_state(
                &self.u_data,
                &mut self.svc,
                charge_key,
                old_a_state,
                new_a_state,
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
