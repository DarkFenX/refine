use lender::{Lender, Lending};

use crate::{
    ad::AAttrId,
    api::{EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> EffectiveMutation<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr<'_>> {
        iter_full_mattrs(self.sol, self.item_uid)
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&self) -> impl Iterator<Item = FullMAttr<'_>> {
        iter_full_mattrs(self.sol, self.item_uid)
    }
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs_mut(&mut self) -> FullMAttrIter<'_> {
        FullMAttrIter::new(self.sol, self.item_uid)
    }
}

// Lending iterator for attribute rolls
pub struct FullMAttrIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    attr_aids: Vec<AAttrId>,
    index: usize,
}
impl<'iter> FullMAttrIter<'iter> {
    pub(in crate::api) fn new(sol: &'iter mut SolarSystem, item_uid: UItemId) -> Self {
        let attr_aids = full_mutated_attr_aid_iter(sol, item_uid).collect();
        Self {
            sol,
            item_uid,
            attr_aids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for FullMAttrIter<'iter> {
    type Lend = FullMAttrMut<'lend>;
}
impl<'iter> Lender for FullMAttrIter<'iter> {
    fn next(&mut self) -> Option<FullMAttrMut<'_>> {
        let attr_aid = *self.attr_aids.get(self.index)?;
        self.index += 1;
        Some(FullMAttrMut::new(self.sol, self.item_uid, attr_aid))
    }
}

fn full_mutated_attr_aid_iter(sol: &SolarSystem, item_uid: UItemId) -> impl Iterator<Item = AAttrId> {
    let u_item = sol.u_data.items.get(item_uid);
    u_item
        .get_mutation_data()
        .unwrap()
        .get_cache()
        .unwrap()
        .get_r_mutator()
        .attr_mods
        .keys()
        .filter_map(|&attr_rid| match u_item.get_attrs().unwrap().contains_key(&attr_rid) {
            true => Some(sol.u_data.src.get_attr_by_rid(attr_rid).aid),
            false => None,
        })
}

fn iter_full_mattrs(sol: &SolarSystem, item_uid: UItemId) -> impl Iterator<Item = FullMAttr<'_>> + use<'_> {
    full_mutated_attr_aid_iter(sol, item_uid).map(move |attr_aid| FullMAttr::new(sol, item_uid, attr_aid))
}
