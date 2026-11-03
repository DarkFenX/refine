use crate::{
    ad::{AAttrId, AEffectId},
    api::EffectId,
    sol::SolarSystem,
    ud::UItemId,
};

/// Side effect of a booster.
pub struct SideEffect<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) item_uid: UItemId,
    pub(super) effect_aid: AEffectId,
    pub(super) chance_attr_aid: AAttrId,
}
impl<'s> SideEffect<'s> {
    pub(in crate::api) fn new(
        sol: &'s mut SolarSystem,
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
    pub fn get_effect_id(&self) -> EffectId {
        EffectId::from_aid(self.effect_aid)
    }
}
