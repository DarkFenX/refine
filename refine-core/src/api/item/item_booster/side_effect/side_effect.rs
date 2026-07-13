use crate::{
    ad::{AAttrId, AEffectId},
    api::EffectId,
    sol::SolarSystem,
    ud::UItemId,
};

/// Side effect of a booster.
pub struct SideEffect<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
    pub(in crate::api) chance_attr_aid: AAttrId,
}
impl<'s> SideEffect<'s> {
    pub(in crate::api) fn new(
        sol: &'s SolarSystem,
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

/// Side effect of a booster.
pub struct SideEffectMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
    pub(in crate::api) chance_attr_aid: AAttrId,
}
impl<'s> SideEffectMut<'s> {
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
