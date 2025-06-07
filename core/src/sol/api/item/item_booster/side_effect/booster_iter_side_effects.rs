use lender::{Lender, Lending};

use super::shared::get_side_effect_chance_attr_id;
use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        api::{Booster, BoosterMut, FullSideEffect, FullSideEffectMut},
    },
};

// Lending iterator for side effects
pub struct SideEffectIter<'iter> {
    sol: &'iter mut SolarSystem,
    key: ItemKey,
    effects_with_chances: Vec<(ad::AEffectId, ad::AAttrId)>,
    index: usize,
}
impl<'iter> SideEffectIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, key: ItemKey, effects_with_chances: Vec<(ad::AEffectId, ad::AAttrId)>) -> Self {
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
        let (a_effect_id, a_attr_id) = *self.effects_with_chances.get(self.index)?;
        self.index += 1;
        Some(FullSideEffectMut::new(self.sol, self.key, a_effect_id, a_attr_id))
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
        let uad_booster = self.sol.uad.items.get(self.key).get_booster().unwrap();
        let effects_with_chances = uad_booster
            .get_a_effect_datas()
            .into_iter()
            .flat_map(|a_effect_datas| {
                a_effect_datas.keys().filter_map(|a_effect_id| {
                    get_side_effect_chance_attr_id(&self.sol.uad.src, a_effect_id)
                        .map(|chance_a_attr_id| (*a_effect_id, chance_a_attr_id))
                })
            })
            .collect();
        SideEffectIter::new(self.sol, self.key, effects_with_chances)
    }
}

fn iter_side_effects(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = FullSideEffect<'_>> {
    let uad_booster = sol.uad.items.get(item_key).get_booster().unwrap();
    uad_booster
        .get_a_effect_datas()
        .into_iter()
        .flat_map(move |a_effect_datas| {
            a_effect_datas.keys().filter_map(move |a_effect_id| {
                get_side_effect_chance_attr_id(&sol.uad.src, a_effect_id)
                    .map(|chance_a_attr_id| FullSideEffect::new(sol, item_key, *a_effect_id, chance_a_attr_id))
            })
        })
}
