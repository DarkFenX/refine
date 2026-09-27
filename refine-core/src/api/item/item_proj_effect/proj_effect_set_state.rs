use crate::{api::ProjEffectMut, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_set_proj_effect_state(&mut self, proj_effect_uid: UItemId, state: bool) {
        let u_proj_effect = self.u_data.items.get_mut(proj_effect_uid).dc_proj_effect_mut().unwrap();
        let old_a_state = u_proj_effect.get_state();
        u_proj_effect.set_proj_effect_state(state);
        let new_a_state = u_proj_effect.get_state();
        u_proj_effect.update_reffs(&mut self.cache.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            proj_effect_uid,
            old_a_state,
            new_a_state,
            &self.cache.eupdates,
        );
    }
}

impl<'s> ProjEffectMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_proj_effect_state(self.uid, state)
    }
}
