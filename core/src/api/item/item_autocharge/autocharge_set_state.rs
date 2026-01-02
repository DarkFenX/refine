use crate::{
    api::AutochargeMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_autocharge_state(
        &mut self,
        autocharge_key: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_autocharge = self.u_data.items.get_mut(autocharge_key).dc_autocharge_mut().unwrap();
        let old_a_state = u_autocharge.get_state();
        u_autocharge.set_force_disabled(!state);
        let new_a_state = u_autocharge.get_state();
        u_autocharge.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            autocharge_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> AutochargeMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_autocharge_state(self.key, state, &mut reuse_eupdates)
    }
}
