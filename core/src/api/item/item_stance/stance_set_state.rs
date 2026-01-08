use crate::{
    api::StanceMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_stance_state(
        &mut self,
        stance_uid: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_stance = self.u_data.items.get_mut(stance_uid).dc_stance_mut().unwrap();
        let old_a_state = u_stance.get_state();
        u_stance.set_stance_state(state);
        let new_a_state = u_stance.get_state();
        u_stance.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            stance_uid,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> StanceMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_set_stance_state(self.uid, state, &mut reuse_eupdates)
    }
}
