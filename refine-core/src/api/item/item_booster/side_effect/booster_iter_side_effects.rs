use lender::{Lender, Lending, check_covariance};

use super::shared::get_se_chance_attr_aid_by_effect_rid;
use crate::{
    ad::{AAttrId, AEffectId},
    api::{Booster, BoosterMut, SideEffect, SideEffectMut},
    sol::SolarSystem,
    ud::UItemId,
};

// Lending iterator for side effects
pub struct SideEffectIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    effects_with_chances: Vec<(AEffectId, AAttrId)>,
    index: usize,
}
impl<'iter> SideEffectIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, item_uid: UItemId, effects_with_chances: Vec<(AEffectId, AAttrId)>) -> Self {
        Self {
            sol,
            item_uid,
            effects_with_chances,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for SideEffectIter<'iter> {
    type Lend = SideEffectMut<'lend>;
}
impl<'iter> Lender for SideEffectIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<SideEffectMut<'_>> {
        let (effect_id, attr_id) = *self.effects_with_chances.get(self.index)?;
        self.index += 1;
        Some(SideEffectMut::new(self.sol, self.item_uid, effect_id, attr_id))
    }
}

impl<'s> Booster<'s> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = SideEffect<'_>> {
        iter_side_effects(self.sol, self.uid)
    }
}

impl<'s> BoosterMut<'s> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = SideEffect<'_>> {
        iter_side_effects(self.sol, self.uid)
    }
    /// Iterates over booster's side effects.
    pub fn iter_side_effects_mut(&mut self) -> SideEffectIter<'_> {
        let u_booster = self.sol.u_data.items.get(self.uid).dc_booster().unwrap();
        let effects_with_chances = u_booster
            .get_effects()
            .into_iter()
            .flat_map(|effects| {
                effects.keys().filter_map(|&effect_rid| {
                    let effect_id = self.sol.u_data.r_data.get_effect_by_rid(effect_rid).aid;
                    get_se_chance_attr_aid_by_effect_rid(&self.sol.u_data.r_data, effect_rid)
                        .map(|chance_attr_id| (effect_id, chance_attr_id))
                })
            })
            .collect();
        SideEffectIter::new(self.sol, self.uid, effects_with_chances)
    }
}

fn iter_side_effects(sol: &SolarSystem, booster_uid: UItemId) -> impl Iterator<Item = SideEffect<'_>> {
    let u_booster = sol.u_data.items.get(booster_uid).dc_booster().unwrap();
    u_booster.get_effects().into_iter().flat_map(move |effect_datas| {
        effect_datas.keys().filter_map(move |&effect_rid| {
            get_se_chance_attr_aid_by_effect_rid(&sol.u_data.r_data, effect_rid).map(|chance_attr_id| {
                SideEffect::new(
                    sol,
                    booster_uid,
                    sol.u_data.r_data.get_effect_by_rid(effect_rid).aid,
                    chance_attr_id,
                )
            })
        })
    })
}
