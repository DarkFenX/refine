use lender::{Lender, Lending, check_covariance};

use crate::{
    ad::AAttrId,
    api::{
        EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut,
        RawMAttr, RawMAttrMut,
    },
    sol::SolarSystem,
    ud::UItemId,
};

impl<'s> Mutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        let (sol, item_uid) = match self {
            Self::Effective(effective_mutation) => (effective_mutation.sol, effective_mutation.item_uid),
            Self::Incomplete(incomplete_mutation) => (incomplete_mutation.sol, incomplete_mutation.item_uid),
        };
        iter_raw_mattrs(sol, item_uid)
    }
}

impl<'s> MutationMut<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        let (sol, item_uid) = match self {
            Self::Effective(effective_mutation) => (&*effective_mutation.sol, effective_mutation.item_uid),
            Self::Incomplete(incomplete_mutation) => (&*incomplete_mutation.sol, incomplete_mutation.item_uid),
        };
        iter_raw_mattrs(sol, item_uid)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter<'_> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.iter_raw_mattrs_mut(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.iter_raw_mattrs_mut(),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        iter_raw_mattrs(self.sol, self.item_uid)
    }
}

impl<'s> EffectiveMutationMut<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        iter_raw_mattrs(self.sol, self.item_uid)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter<'_> {
        RawMAttrIter::new(self.sol, self.item_uid)
    }
}

impl<'s> IncompleteMutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        iter_raw_mattrs(self.sol, self.item_uid)
    }
}

impl<'s> IncompleteMutationMut<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&self) -> impl ExactSizeIterator<Item = RawMAttr<'_>> {
        iter_raw_mattrs(self.sol, self.item_uid)
    }
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs_mut(&mut self) -> RawMAttrIter<'_> {
        RawMAttrIter::new(self.sol, self.item_uid)
    }
}

// Lending iterator for attribute rolls
pub struct RawMAttrIter<'iter> {
    sol: &'iter mut SolarSystem,
    item_uid: UItemId,
    attr_aids: Vec<AAttrId>,
    index: usize,
}
impl<'iter> RawMAttrIter<'iter> {
    pub(in crate::api) fn new(sol: &'iter mut SolarSystem, item_uid: UItemId) -> Self {
        let attr_aids = raw_mutated_attr_id_iter(sol, item_uid).collect();
        Self {
            sol,
            item_uid,
            attr_aids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for RawMAttrIter<'iter> {
    type Lend = RawMAttrMut<'lend>;
}
impl<'iter> Lender for RawMAttrIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<RawMAttrMut<'_>> {
        let attr_aid = *self.attr_aids.get(self.index)?;
        self.index += 1;
        Some(RawMAttrMut::new(self.sol, self.item_uid, attr_aid))
    }
}

fn raw_mutated_attr_id_iter(sol: &SolarSystem, item_uid: UItemId) -> impl ExactSizeIterator<Item = AAttrId> {
    sol.u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .keys()
        .copied()
}

fn iter_raw_mattrs(sol: &SolarSystem, item_uid: UItemId) -> impl ExactSizeIterator<Item = RawMAttr<'_>> + use<'_> {
    raw_mutated_attr_id_iter(sol, item_uid).map(move |attr_id| RawMAttr::new(sol, item_uid, attr_id))
}
