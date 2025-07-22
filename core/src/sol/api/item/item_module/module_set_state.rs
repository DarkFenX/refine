use crate::{
    misc::ModuleState,
    sol::{SolarSystem, api::ModuleMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_state(
        &mut self,
        item_key: UadItemKey,
        state: ModuleState,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        // Update user data for module
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = uad_module.get_charge_key();
        let old_a_state = uad_module.get_a_state();
        uad_module.set_module_state(state, reuse_eupdates, &self.uad.src);
        let new_a_state = uad_module.get_a_state();
        // Update services for module
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            let old_a_state = uad_charge.get_a_state();
            uad_charge.set_a_state(state.into(), reuse_eupdates, &self.uad.src);
            let new_a_state = uad_charge.get_a_state();
            // Update services for charge
            SolarSystem::util_switch_item_state(
                &self.uad,
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
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_set_module_state(self.key, state, &mut reuse_eupdates)
    }
}
