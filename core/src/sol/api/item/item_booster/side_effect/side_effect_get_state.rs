use crate::{
    ad,
    def::ItemKey,
    misc::EffectMode,
    sol::{
        SolarSystem,
        api::{FullSideEffect, FullSideEffectMut, SideEffect, SideEffectMut, StubSideEffect, StubSideEffectMut},
    },
};

impl<'a> SideEffect<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        match self {
            Self::Full(full_side_effect) => full_side_effect.get_state(),
            Self::Stub(stub_side_effect) => stub_side_effect.get_state(),
        }
    }
}

impl<'a> SideEffectMut<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        match self {
            Self::Full(full_side_effect) => full_side_effect.get_state(),
            Self::Stub(stub_side_effect) => stub_side_effect.get_state(),
        }
    }
}

impl<'a> FullSideEffect<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.a_effect_id)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.a_effect_id)
    }
}

impl<'a> StubSideEffect<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.a_effect_id)
    }
}

impl<'a> StubSideEffectMut<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.a_effect_id)
    }
}

fn get_state(sol: &SolarSystem, item_key: ItemKey, a_effect_id: &ad::AEffectId) -> bool {
    let uad_booster = sol.uad.items.get(item_key).get_booster().unwrap();
    match uad_booster.get_effect_mode(a_effect_id) {
        EffectMode::FullCompliance => false,
        EffectMode::StateCompliance => true,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}
