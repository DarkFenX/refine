use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use super::shared::get_se_chance_attr_aid_by_effect_rid;
use crate::{
    Booster, SideEffect,
    ad::{AAttrId, AEffectId},
    sol::CoreInner,
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Lending iterator for side effects
pub struct SideEffectIter<'iter> {
    core: &'iter mut CoreInner,
    item_uid: UItemId,
    effects_with_chances: Vec<(AEffectId, AAttrId)>,
    index: usize,
}
impl<'iter, 'lend> Lending<'lend> for SideEffectIter<'iter> {
    type Lend = SideEffect<'lend>;
}
impl<'iter> Lender for SideEffectIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<SideEffect<'_>> {
        let (effect_id, attr_id) = *self.effects_with_chances.get(self.index)?;
        self.index += 1;
        Some(SideEffect::new(self.core, self.item_uid, effect_id, attr_id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.effects_with_chances.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for SideEffectIter<'iter> {}

impl<'s> Booster<'s> {
    /// Iterates over booster's side effects.
    pub fn iter_side_effects(&mut self) -> SideEffectIter<'_> {
        let u_booster = self.core.u_data.items.get(self.uid).dc_booster().unwrap();
        let effects_with_chances = u_booster
            .get_r_item_base()
            .into_iter()
            .flat_map(|rib| {
                rib.effects.keys().filter_map(|&effect_rid| {
                    let effect_id = self.core.u_data.r_data.get_effect_by_rid(effect_rid).aid;
                    get_se_chance_attr_aid_by_effect_rid(&self.core.u_data.r_data, effect_rid)
                        .map(|chance_attr_id| (effect_id, chance_attr_id))
                })
            })
            .collect();
        SideEffectIter::new(self.core, self.uid, effects_with_chances)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'iter> SideEffectIter<'iter> {
    fn new(core: &'iter mut CoreInner, item_uid: UItemId, effects_with_chances: Vec<(AEffectId, AAttrId)>) -> Self {
        Self {
            core,
            item_uid,
            effects_with_chances,
            index: 0,
        }
    }
}
