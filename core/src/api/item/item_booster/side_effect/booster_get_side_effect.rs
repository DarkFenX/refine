// You can get any effect as a side effect, this is by design, to be able to change side effect
// status on a source which doesn't have an effect defined as a side effect.

use super::shared::get_se_chance_attr_aid_by_effect_aid;
use crate::{
    api::{
        Booster, BoosterMut, EffectId, FullSideEffect, FullSideEffectMut, SideEffect, SideEffectMut, StubSideEffect,
        StubSideEffectMut,
    },
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Booster<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> SideEffect<'_> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
}

impl<'a> BoosterMut<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> SideEffect<'_> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
    pub fn get_side_effect_mut(&mut self, effect_id: &EffectId) -> SideEffectMut<'_> {
        let effect_aid = effect_id.into_aid();
        match get_se_chance_attr_aid_by_effect_aid(&self.sol.u_data.src, &effect_aid) {
            Some(chance_attr_aid) => {
                SideEffectMut::Full(FullSideEffectMut::new(self.sol, self.uid, effect_aid, chance_attr_aid))
            }
            None => SideEffectMut::Stub(StubSideEffectMut::new(self.sol, self.uid, effect_aid)),
        }
    }
}

fn get_side_effect<'a>(sol: &'a SolarSystem, booster_uid: UItemId, effect_id: &EffectId) -> SideEffect<'a> {
    let effect_aid = effect_id.into_aid();
    match get_se_chance_attr_aid_by_effect_aid(&sol.u_data.src, &effect_aid) {
        Some(chance_attr_aid) => SideEffect::Full(FullSideEffect::new(sol, booster_uid, effect_aid, chance_attr_aid)),
        None => SideEffect::Stub(StubSideEffect::new(sol, booster_uid, effect_aid)),
    }
}
