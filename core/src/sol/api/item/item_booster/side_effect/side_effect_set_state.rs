use crate::{
    ad::AEffectId,
    misc::EffectMode,
    sol::{
        SolarSystem,
        api::{FullSideEffectMut, SideEffectMut, StubSideEffectMut},
    },
    ud::{UEffectUpdates, UItemKey},
};

impl<'a> SideEffectMut<'a> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        match self {
            Self::Full(full_side_effect) => full_side_effect.set_state(state),
            Self::Stub(stub_side_effect) => stub_side_effect.set_state(state),
        }
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        set_state(self.sol, self.key, self.effect_id, state)
    }
}

impl<'a> StubSideEffectMut<'a> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        set_state(self.sol, self.key, self.effect_id, state)
    }
}

fn set_state(sol: &mut SolarSystem, item_key: UItemKey, a_effect_id: AEffectId, state: bool) {
    let u_booster = sol.u_data.items.get_mut(item_key).get_booster_mut().unwrap();
    let effect_mode = match state {
        true => EffectMode::StateCompliance,
        false => EffectMode::FullCompliance,
    };
    let mut reuse_eupdates = UEffectUpdates::new();
    u_booster.set_effect_mode(a_effect_id, effect_mode, &sol.u_data.src);
    u_booster.update_reffs(&mut reuse_eupdates, &sol.u_data.src);
    SolarSystem::util_process_effect_updates(&sol.u_data, &mut sol.svc, item_key, &reuse_eupdates);
}
