use crate::{
    ad::AEffectId,
    misc::EffectMode,
    sol::{
        SolarSystem,
        api::{FullSideEffect, FullSideEffectMut, SideEffect, SideEffectMut, StubSideEffect, StubSideEffectMut},
    },
    ud::UItemKey,
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
        get_state(self.sol, self.key, &self.effect_id)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.effect_id)
    }
}

impl<'a> StubSideEffect<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.effect_id)
    }
}

impl<'a> StubSideEffectMut<'a> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key, &self.effect_id)
    }
}

fn get_state(sol: &SolarSystem, booster_key: UItemKey, effect_id: &AEffectId) -> bool {
    let u_booster = sol.u_data.items.get(booster_key).dc_booster().unwrap();
    match u_booster.get_effect_id_mode(effect_id) {
        EffectMode::FullCompliance => false,
        EffectMode::StateCompliance => true,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}
