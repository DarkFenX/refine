use lender::{Lender, Lending};

use super::shared::get_se_chance_attr_id_by_effect_rid;
use crate::{
    ad::{AAttrId, AEffectId},
    api::{Booster, BoosterMut, FullSideEffect, FullSideEffectMut},
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
    type Lend = FullSideEffectMut<'lend>;
}
impl<'iter> Lender for SideEffectIter<'iter> {
    fn next(&mut self) -> Option<FullSideEffectMut<'_>> {
        let (effect_id, attr_id) = *self.effects_with_chances.get(self.index)?;
        self.index += 1;
        Some(FullSideEffectMut::new(self.sol, self.item_uid, effect_id, attr_id))
    }
}

impl<'a> Booster<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect<'_>> {
        iter_side_effects(self.sol, self.uid)
    }
}

impl<'a> BoosterMut<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect<'_>> {
        iter_side_effects(self.sol, self.uid)
    }
    /// Iterates over booster's side effects.
    pub fn iter_side_effects_mut(&mut self) -> SideEffectIter<'_> {
        let u_booster = self.sol.u_data.items.get(self.uid).dc_booster().unwrap();
        let effects_with_chances = u_booster
            .get_effect_datas()
            .into_iter()
            .flat_map(|effect_datas| {
                effect_datas.keys().filter_map(|&effect_rid| {
                    let effect_id = self.sol.u_data.src.get_effect_by_rid(effect_rid).aid;
                    get_se_chance_attr_id_by_effect_rid(&self.sol.u_data.src, effect_rid)
                        .map(|chance_attr_id| (effect_id, chance_attr_id))
                })
            })
            .collect();
        SideEffectIter::new(self.sol, self.uid, effects_with_chances)
    }
}

fn iter_side_effects(sol: &SolarSystem, booster_uid: UItemId) -> impl Iterator<Item = FullSideEffect<'_>> {
    let u_booster = sol.u_data.items.get(booster_uid).dc_booster().unwrap();
    u_booster.get_effect_datas().into_iter().flat_map(move |effect_datas| {
        effect_datas.keys().filter_map(move |&effect_rid| {
            get_se_chance_attr_id_by_effect_rid(&sol.u_data.src, effect_rid).map(|chance_attr_id| {
                FullSideEffect::new(
                    sol,
                    booster_uid,
                    sol.u_data.src.get_effect_by_rid(effect_rid).aid,
                    chance_attr_id,
                )
            })
        })
    })
}
