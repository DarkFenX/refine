use std::collections::VecDeque;

use lender::{Lender, Lending};

use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        api::{Booster, BoosterMut, FullSideEffect, FullSideEffectMut},
    },
};

use super::shared::get_side_effect_chance_attr_id;

// Lending iterator for side effects
pub struct SideEffectIter<'this> {
    sol: &'this mut SolarSystem,
    key: ItemKey,
    effects_with_chances: VecDeque<(ad::AEffectId, ad::AAttrId)>,
}
impl<'this, 'lend> Lending<'lend> for SideEffectIter<'this> {
    type Lend = FullSideEffectMut<'lend>;
}
impl<'this> Lender for SideEffectIter<'this> {
    fn next(&mut self) -> Option<FullSideEffectMut> {
        let (a_effect_id, a_attr_id) = self.effects_with_chances.pop_front()?;
        Some(FullSideEffectMut::new(self.sol, self.key, a_effect_id, a_attr_id))
    }
}

impl<'a> Booster<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect> {
        iter_side_effects(self.sol, self.key)
    }
}

impl<'a> BoosterMut<'a> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&self) -> impl Iterator<Item = FullSideEffect> {
        iter_side_effects(self.sol, self.key)
    }
    /// Iterates over booster's side effects.
    pub fn iter_side_effects_mut(&mut self) -> SideEffectIter {
        let uad_booster = self.sol.uad.items.get(self.key).get_booster().unwrap();
        let effects_with_chances = uad_booster
            .get_a_effect_datas()
            .into_iter()
            .map(|a_effect_datas| {
                a_effect_datas.keys().filter_map(|a_effect_id| {
                    get_side_effect_chance_attr_id(&self.sol.uad.src, a_effect_id)
                        .map(|chance_a_attr_id| (*a_effect_id, chance_a_attr_id))
                })
            })
            .flatten()
            .collect();
        SideEffectIter {
            sol: self.sol,
            key: self.key,
            effects_with_chances,
        }
    }
}

fn iter_side_effects(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = FullSideEffect> {
    let uad_booster = sol.uad.items.get(item_key).get_booster().unwrap();
    uad_booster
        .get_a_effect_datas()
        .into_iter()
        .map(move |a_effect_datas| {
            a_effect_datas.keys().filter_map(move |a_effect_id| {
                get_side_effect_chance_attr_id(&sol.uad.src, a_effect_id)
                    .map(|chance_a_attr_id| FullSideEffect::new(sol, item_key, *a_effect_id, chance_a_attr_id))
            })
        })
        .flatten()
}
