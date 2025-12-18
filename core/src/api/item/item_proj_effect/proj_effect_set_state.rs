use crate::{
    api::ProjEffectMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_proj_effect_state(
        &mut self,
        proj_effect_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_proj_effect = self.u_data.items.get_mut(proj_effect_key).dc_proj_effect_mut().unwrap();
        let old_a_state = u_proj_effect.get_state();
        u_proj_effect.set_proj_effect_state(state);
        let new_a_state = u_proj_effect.get_state();
        u_proj_effect.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            proj_effect_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> ProjEffectMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_proj_effect_state(self.key, state, &mut reuse_eupdates)
    }
}
