use super::shared::get_se_chance_attr_aid_by_effect_aid;
use crate::{
    api::{Booster, BoosterMut, EffectId, SideEffect, SideEffectMut},
    err::basic::SideEffectFoundError,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> Booster<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> Result<SideEffect<'_>, GetSideEffectError> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
}

impl<'a> BoosterMut<'a> {
    pub fn get_side_effect(&self, effect_id: &EffectId) -> Result<SideEffect<'_>, GetSideEffectError> {
        get_side_effect(self.sol, self.uid, effect_id)
    }
    pub fn get_side_effect_mut(&mut self, effect_id: &EffectId) -> Result<SideEffectMut<'_>, GetSideEffectError> {
        let effect_aid = effect_id.into_aid();
        match get_se_chance_attr_aid_by_effect_aid(&self.sol.u_data.src, &effect_aid) {
            Some(chance_attr_aid) => Ok(SideEffectMut::new(self.sol, self.uid, effect_aid, chance_attr_aid)),
            None => Err(SideEffectFoundError {
                item_id: self.sol.u_data.items.xid_by_iid(self.uid),
                effect_id: *effect_id,
            }
            .into()),
        }
    }
}

fn get_side_effect<'a>(
    sol: &'a SolarSystem,
    booster_uid: UItemId,
    effect_id: &EffectId,
) -> Result<SideEffect<'a>, GetSideEffectError> {
    let effect_aid = effect_id.into_aid();
    match get_se_chance_attr_aid_by_effect_aid(&sol.u_data.src, &effect_aid) {
        Some(chance_attr_aid) => Ok(SideEffect::new(sol, booster_uid, effect_aid, chance_attr_aid)),
        None => Err(SideEffectFoundError {
            item_id: sol.u_data.items.xid_by_iid(booster_uid),
            effect_id: *effect_id,
        }
        .into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSideEffectError {
    #[error("{0}")]
    SideEffectNotFound(#[from] SideEffectFoundError),
}
