use crate::{
    EffectId,
    ad::{AAttrId, AEffectId},
    sol::SolCore,
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Side effect of a booster.
pub struct SideEffect<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) item_uid: UItemId,
    pub(super) effect_aid: AEffectId,
    pub(super) chance_attr_aid: AAttrId,
}
impl<'sol> SideEffect<'sol> {
    pub fn get_effect_id(&self) -> EffectId {
        EffectId::from_aid(self.effect_aid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> SideEffect<'sol> {
    pub(in crate::api) fn new(
        sol: &'sol mut SolCore,
        item_uid: UItemId,
        effect_aid: AEffectId,
        chance_attr_aid: AAttrId,
    ) -> Self {
        Self {
            sol,
            item_uid,
            effect_aid,
            chance_attr_aid,
        }
    }
}
