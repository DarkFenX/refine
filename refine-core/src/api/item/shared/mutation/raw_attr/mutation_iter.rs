use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{DormantMutation, EffectiveMutation, Mutation, RawMAttr, ad::AAttrId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Mutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&mut self) -> RawMAttrIter<'_> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.iter_raw_mattrs(),
            Self::Dormant(dormant_mutation) => dormant_mutation.iter_raw_mattrs(),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&mut self) -> RawMAttrIter<'_> {
        RawMAttrIter::new(self.core, self.item_uid)
    }
}

impl<'s> DormantMutation<'s> {
    /// Iterates over mutation's raw mutated attributes.
    pub fn iter_raw_mattrs(&mut self) -> RawMAttrIter<'_> {
        RawMAttrIter::new(self.core, self.item_uid)
    }
}

// Lending iterator for attribute rolls
pub struct RawMAttrIter<'iter> {
    core: &'iter mut CoreInner,
    item_uid: UItemId,
    attr_aids: Vec<AAttrId>,
    index: usize,
}
impl<'iter, 'lend> Lending<'lend> for RawMAttrIter<'iter> {
    type Lend = RawMAttr<'lend>;
}
impl<'iter> Lender for RawMAttrIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<RawMAttr<'_>> {
        let attr_aid = *self.attr_aids.get(self.index)?;
        self.index += 1;
        Some(RawMAttr::new(self.core, self.item_uid, attr_aid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.attr_aids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for RawMAttrIter<'iter> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'iter> RawMAttrIter<'iter> {
    pub(in crate::api) fn new(core: &'iter mut CoreInner, item_uid: UItemId) -> Self {
        let attr_aids = core
            .u_data
            .items
            .get(item_uid)
            .get_mutation_data()
            .unwrap()
            .get_attr_rolls()
            .keys()
            .copied()
            .collect();
        Self {
            core,
            item_uid,
            attr_aids,
            index: 0,
        }
    }
}
