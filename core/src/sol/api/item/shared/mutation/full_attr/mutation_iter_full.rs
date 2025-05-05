use lender::{Lender, Lending};

use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        api::{EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    },
};

impl<'a> EffectiveMutation<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr> {
        iter_full_mattrs(self.sol, self.item_key)
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr> {
        iter_full_mattrs(self.sol, self.item_key)
    }
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs_mut(&mut self) -> FullMAttrIter {
        FullMAttrIter::new(self.sol, self.item_key)
    }
}

// Lending iterator for attribute rolls
pub struct FullMAttrIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_key: ItemKey,
    a_attr_ids: Vec<ad::AAttrId>,
    index: usize,
}
impl<'iter> FullMAttrIter<'iter> {
    pub(in crate::sol::api) fn new(sol: &'iter mut SolarSystem, item_key: ItemKey) -> Self {
        let a_attr_ids = full_mutated_a_attr_id_iter(sol, item_key).collect();
        Self {
            sol,
            item_key,
            a_attr_ids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for FullMAttrIter<'iter> {
    type Lend = FullMAttrMut<'lend>;
}
impl<'iter> Lender for FullMAttrIter<'iter> {
    fn next(&mut self) -> Option<FullMAttrMut> {
        let a_attr_id = *self.a_attr_ids.get(self.index)?;
        self.index += 1;
        Some(FullMAttrMut::new(self.sol, self.item_key, a_attr_id))
    }
}

fn full_mutated_a_attr_id_iter(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = ad::AAttrId> {
    let uad_item = sol.uad.items.get(item_key);
    uad_item
        .get_mutation_data()
        .unwrap()
        .get_cache()
        .unwrap()
        .get_a_mutator()
        .attr_mods
        .keys()
        .filter(|v| uad_item.get_a_attrs().unwrap().contains_key(v))
        .copied()
}

fn iter_full_mattrs(sol: &SolarSystem, item_key: ItemKey) -> impl Iterator<Item = FullMAttr> + use<'_> {
    full_mutated_a_attr_id_iter(sol, item_key).map(move |a_attr_id| FullMAttr::new(sol, item_key, a_attr_id))
}
