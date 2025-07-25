// You can get any effect as a side effect, this is by design, to be able to change side effect
// status on a source which doesn't have an effect defined as a side effect.

use super::shared::get_side_effect_chance_attr_id;
use crate::{
    ad,
    misc::EffectId,
    sol::{
        SolarSystem,
        api::{
            Booster, BoosterMut, FullSideEffect, FullSideEffectMut, SideEffect, SideEffectMut, StubSideEffect,
            StubSideEffectMut,
        },
    },
    ud::UItemKey,
};

impl<'a> Booster<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> SideEffect<'_> {
        get_side_effect(self.sol, self.key, effect_id)
    }
}

impl<'a> BoosterMut<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> SideEffect<'_> {
        get_side_effect(self.sol, self.key, effect_id)
    }
    pub fn get_side_effect_mut(&mut self, effect_id: &EffectId) -> SideEffectMut<'_> {
        let a_effect_id = ad::AEffectId::from(effect_id);
        match get_side_effect_chance_attr_id(&self.sol.u_data.src, &a_effect_id) {
            Some(chance_a_attr_id) => SideEffectMut::Full(FullSideEffectMut::new(
                self.sol,
                self.key,
                a_effect_id,
                chance_a_attr_id,
            )),
            None => SideEffectMut::Stub(StubSideEffectMut::new(self.sol, self.key, a_effect_id)),
        }
    }
}

fn get_side_effect<'a>(sol: &'a SolarSystem, item_key: UItemKey, effect_id: &EffectId) -> SideEffect<'a> {
    let a_effect_id = ad::AEffectId::from(effect_id);
    match get_side_effect_chance_attr_id(&sol.u_data.src, &a_effect_id) {
        Some(chance_a_attr_id) => SideEffect::Full(FullSideEffect::new(sol, item_key, a_effect_id, chance_a_attr_id)),
        None => SideEffect::Stub(StubSideEffect::new(sol, item_key, a_effect_id)),
    }
}
