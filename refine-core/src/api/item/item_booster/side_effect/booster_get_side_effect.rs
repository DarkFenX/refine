use super::shared::get_se_chance_attr_aid_by_effect_aid;
use crate::{Booster, BoosterMut, EffectId, ItemId, SideEffect, SideEffectMut, SolarSystem, ud::UItemId};

impl<'s> Booster<'s> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> Result<SideEffect<'_>, GetSideEffectError> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
}

impl<'s> BoosterMut<'s> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> Result<SideEffect<'_>, GetSideEffectError> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
    pub fn get_side_effect_mut(&mut self, effect_id: &EffectId) -> Result<SideEffectMut<'_>, GetSideEffectError> {
        let effect_aid = effect_id.into_aid();
        match get_se_chance_attr_aid_by_effect_aid(&self.sol.u_data.r_data, &effect_aid) {
            Some(chance_attr_aid) => Ok(SideEffectMut::new(self.sol, self.uid, effect_aid, chance_attr_aid)),
            None => Err(GetSideEffectError::SideEffectNotFound(
                self.sol.u_data.items.ext_id_by_int_id(self.uid),
                *effect_id,
            )),
        }
    }
}

fn get_side_effect<'s>(
    sol: &'s SolarSystem,
    booster_uid: UItemId,
    effect_id: &EffectId,
) -> Result<SideEffect<'s>, GetSideEffectError> {
    let effect_aid = effect_id.into_aid();
    match get_se_chance_attr_aid_by_effect_aid(&sol.u_data.r_data, &effect_aid) {
        Some(chance_attr_aid) => Ok(SideEffect::new(sol, booster_uid, effect_aid, chance_attr_aid)),
        None => Err(GetSideEffectError::SideEffectNotFound(
            sol.u_data.items.ext_id_by_int_id(booster_uid),
            *effect_id,
        )),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetSideEffectError {
    #[error("effect {1} is not a side effect on item {0}")]
    SideEffectNotFound(ItemId, EffectId),
}
