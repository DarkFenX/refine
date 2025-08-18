use crate::{
    sol::{SolarSystem, api::SubsystemMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_subsystem_state(
        &mut self,
        subsystem_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_subsystem = self.u_data.items.get_mut(subsystem_key).get_subsystem_mut().unwrap();
        let old_a_state = u_subsystem.get_state();
        u_subsystem.set_subsystem_state(state);
        let new_a_state = u_subsystem.get_state();
        u_subsystem.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            subsystem_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> SubsystemMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_subsystem_state(self.key, state, &mut reuse_eupdates)
    }
}
