use lender::{Lender, Lending};

use super::shared::get_se_chance_attr_id_by_effect_key;
use crate::{
    ad::{AAttrId, AEffectId},
    api::{Booster, BoosterMut, FullSideEffect, FullSideEffectMut},
    sol::SolarSystem,
    ud::UItemKey,
};

// Lending iterator for side effects
pub struct SideEffectIter<'iter> {
    sol: &'iter mut SolarSystem,
    key: UItemKey,
    effects_with_chances: Vec<(AEffectId, AAttrId)>,
    index: usize,
}
impl<'iter> SideEffectIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, key: UItemKey, effects_with_chances: Vec<(AEffectId, AAttrId)>) -> Self {
        Self {
            sol,
            key,
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
        Some(FullSideEffectMut::new(self.sol, self.key, effect_id, attr_id))
    }
}

impl<'a> Booster<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect<'_>> {
        iter_side_effects(self.sol, self.key)
    }
}

impl<'a> BoosterMut<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect<'_>> {
        iter_side_effects(self.sol, self.key)
    }
    /// Iterates over booster's side effects.
    pub fn iter_side_effects_mut(&mut self) -> SideEffectIter<'_> {
        let u_booster = self.sol.u_data.items.get(self.key).dc_booster().unwrap();
        let effects_with_chances = u_booster
            .get_effect_datas()
            .into_iter()
            .flat_map(|effect_datas| {
                effect_datas.keys().filter_map(|&effect_key| {
                    let effect_id = self.sol.u_data.src.get_effect(effect_key).id;
                    get_se_chance_attr_id_by_effect_key(&self.sol.u_data.src, effect_key)
                        .map(|chance_attr_id| (effect_id, chance_attr_id))
                })
            })
            .collect();
        SideEffectIter::new(self.sol, self.key, effects_with_chances)
    }
}

fn iter_side_effects(sol: &SolarSystem, booster_key: UItemKey) -> impl Iterator<Item = FullSideEffect<'_>> {
    let u_booster = sol.u_data.items.get(booster_key).dc_booster().unwrap();
    u_booster.get_effect_datas().into_iter().flat_map(move |effect_datas| {
        effect_datas.keys().filter_map(move |&effect_key| {
            get_se_chance_attr_id_by_effect_key(&sol.u_data.src, effect_key).map(|chance_attr_id| {
                FullSideEffect::new(
                    sol,
                    booster_key,
                    sol.u_data.src.get_effect(effect_key).id,
                    chance_attr_id,
                )
            })
        })
    })
}
