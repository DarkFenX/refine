use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{EffectiveMutation, FullMAttr, ad::AAttrId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> EffectiveMutation<'s> {
    /// Iterates over mutation's full mutated attributes.
    pub fn iter_full_mattrs(&mut self) -> FullMAttrIter<'_> {
        FullMAttrIter::new(self.core, self.item_uid)
    }
}

// Lending iterator for attribute rolls
pub struct FullMAttrIter<'iter> {
    core: &'iter mut CoreInner,
    item_uid: UItemId,
    attr_aids: Vec<AAttrId>,
    index: usize,
}
impl<'iter, 'lend> Lending<'lend> for FullMAttrIter<'iter> {
    type Lend = FullMAttr<'lend>;
}
impl<'iter> Lender for FullMAttrIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<FullMAttr<'_>> {
        let attr_aid = *self.attr_aids.get(self.index)?;
        self.index += 1;
        Some(FullMAttr::new(self.core, self.item_uid, attr_aid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.attr_aids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for FullMAttrIter<'iter> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'iter> FullMAttrIter<'iter> {
    pub(in crate::api) fn new(core: &'iter mut CoreInner, item_uid: UItemId) -> Self {
        let u_item = core.u_data.items.get(item_uid);
        let attr_aids = u_item
            .get_mutation_data()
            .unwrap()
            .get_cache()
            .unwrap()
            .get_r_mutator()
            .attr_mods
            .keys()
            .filter_map(
                |&attr_rid| match u_item.get_r_item_flex_data().unwrap().attrs.contains_key(&attr_rid) {
                    true => Some(core.u_data.r_data.get_attr_by_rid(attr_rid).aid),
                    false => None,
                },
            )
            .collect();
        Self {
            core,
            item_uid,
            attr_aids,
            index: 0,
        }
    }
}
