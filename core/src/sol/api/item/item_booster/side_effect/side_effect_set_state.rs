use crate::{
    ad,
    sol::{
        EffectMode, ItemKey, SolarSystem,
        api::{FullSideEffectMut, SideEffectMut, StubSideEffectMut},
    },
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
        set_state(self.sol, self.key, self.a_effect_id, state)
    }
}

impl<'a> StubSideEffectMut<'a> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        set_state(self.sol, self.key, self.a_effect_id, state)
    }
}

fn set_state(sol: &mut SolarSystem, item_key: ItemKey, a_effect_id: ad::AEffectId, state: bool) {
    let uad_booster = sol.uad.items.get_mut(item_key).get_booster_mut().unwrap();
    let effect_state = match state {
        true => EffectMode::StateCompliance,
        false => EffectMode::FullCompliance,
    };
    uad_booster.get_effect_modes_mut().set(a_effect_id, effect_state);
    let uad_item = sol.uad.items.get(item_key);
    SolarSystem::util_process_effects(
        &sol.uad,
        &mut sol.svc,
        &mut sol.reffs,
        item_key,
        uad_item,
        uad_item.get_a_state(),
    );
}
