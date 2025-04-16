use crate::sol::{
    ItemKey, SolarSystem,
    api::{Booster, BoosterMut, FullSideEffect},
};

use super::shared::get_side_effect_chance_attr_id;

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
