use crate::{api::SideEffectMut, misc::EffectMode, sol::SolarSystem, ud::UEffectUpdates};

impl<'a> SideEffectMut<'a> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        let u_booster = self.sol.u_data.items.get_mut(self.item_uid).dc_booster_mut().unwrap();
        let effect_mode = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::FullCompliance,
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        u_booster.set_effect_mode(self.effect_aid, effect_mode, &self.sol.u_data.r_data);
        u_booster.update_reffs(&mut reuse_eupdates, &self.sol.u_data.r_data);
        SolarSystem::util_process_effect_updates(&self.sol.u_data, &mut self.sol.svc, self.item_uid, &reuse_eupdates);
    }
}
