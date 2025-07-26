use lender::{Lender, Lending};

use crate::{
    ad,
    sol::{
        SolarSystem,
        api::{EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    },
    ud::UItemKey,
};

impl<'a> EffectiveMutation<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr<'_>> {
        iter_full_mattrs(self.sol, self.item_key)
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr<'_>> {
        iter_full_mattrs(self.sol, self.item_key)
    }
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs_mut(&mut self) -> FullMAttrIter<'_> {
        FullMAttrIter::new(self.sol, self.item_key)
    }
}

// Lending iterator for attribute rolls
pub struct FullMAttrIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_key: UItemKey,
    a_attr_ids: Vec<ad::AAttrId>,
    index: usize,
}
impl<'iter> FullMAttrIter<'iter> {
    pub(in crate::sol::api) fn new(sol: &'iter mut SolarSystem, item_key: UItemKey) -> Self {
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
    fn next(&mut self) -> Option<FullMAttrMut<'_>> {
        let a_attr_id = *self.a_attr_ids.get(self.index)?;
        self.index += 1;
        Some(FullMAttrMut::new(self.sol, self.item_key, a_attr_id))
    }
}

fn full_mutated_a_attr_id_iter(sol: &SolarSystem, item_key: UItemKey) -> impl Iterator<Item = ad::AAttrId> {
    let u_item = sol.u_data.items.get(item_key);
    u_item
        .get_mutation_data()
        .unwrap()
        .get_cache()
        .unwrap()
        .get_r_mutator()
        .get_attr_mods()
        .keys()
        .filter(|v| u_item.get_attrs().unwrap().contains_key(v))
        .copied()
}

fn iter_full_mattrs(sol: &SolarSystem, item_key: UItemKey) -> impl Iterator<Item = FullMAttr<'_>> + use<'_> {
    full_mutated_a_attr_id_iter(sol, item_key).map(move |a_attr_id| FullMAttr::new(sol, item_key, a_attr_id))
}
