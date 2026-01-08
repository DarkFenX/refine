use crate::{
    ad::{AAttrId, AEffectId},
    api::EffectId,
    sol::SolarSystem,
    ud::UItemId,
};

pub enum SideEffect<'a> {
    Full(FullSideEffect<'a>),
    Stub(StubSideEffect<'a>),
}
impl<'a> SideEffect<'a> {
    pub fn get_effect_id(&self) -> EffectId {
        match self {
            Self::Full(full_side_effect) => full_side_effect.get_effect_id(),
            Self::Stub(stub_side_effect) => stub_side_effect.get_effect_id(),
        }
    }
}

pub enum SideEffectMut<'a> {
    Full(FullSideEffectMut<'a>),
    Stub(StubSideEffectMut<'a>),
}

/// Side effect which has full functionality available.
pub struct FullSideEffect<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
    pub(in crate::api) chance_attr_aid: AAttrId,
}
impl<'a> FullSideEffect<'a> {
    pub(in crate::api) fn new(
        sol: &'a SolarSystem,
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

/// Side effect which has full functionality available.
pub struct FullSideEffectMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
    pub(in crate::api) chance_attr_aid: AAttrId,
}
impl<'a> FullSideEffectMut<'a> {
    pub(in crate::api) fn new(
        sol: &'a mut SolarSystem,
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

/// A non-side-effect effect, with limited functionality. Exists to expose and let manipulate side
/// effect data on data sources which don't have this effect defined as a side effect.
pub struct StubSideEffect<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
}
impl<'a> StubSideEffect<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, item_uid: UItemId, effect_aid: AEffectId) -> Self {
        Self {
            sol,
            item_uid,
            effect_aid,
        }
    }
    pub fn get_effect_id(&self) -> EffectId {
        EffectId::from_aid(self.effect_aid)
    }
}

/// A non-side-effect effect, with limited functionality. Exists to expose and let manipulate side
/// effect data on data sources which don't have this effect defined as a side effect.
pub struct StubSideEffectMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) effect_aid: AEffectId,
}
impl<'a> StubSideEffectMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, item_uid: UItemId, effect_aid: AEffectId) -> Self {
        Self {
            sol,
            item_uid,
            effect_aid,
        }
    }
    pub fn get_effect_id(&self) -> EffectId {
        EffectId::from_aid(self.effect_aid)
    }
}
