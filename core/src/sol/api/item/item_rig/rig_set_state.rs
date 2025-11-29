use crate::{
    sol::{SolarSystem, api::RigMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_rig_state(
        &mut self,
        rig_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_rig = self.u_data.items.get_mut(rig_key).dc_rig_mut().unwrap();
        let old_a_state = u_rig.get_state();
        u_rig.set_rig_state(state);
        let new_a_state = u_rig.get_state();
        u_rig.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            rig_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> RigMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_rig_state(self.key, state, &mut reuse_eupdates)
    }
}
