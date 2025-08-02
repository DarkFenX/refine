use crate::{
    ad::{AAttrId, AEffectId},
    misc::EffectId,
    sol::SolarSystem,
    ud::UItemKey,
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
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
    pub(in crate::sol::api) effect_id: AEffectId,
    pub(in crate::sol::api) chance_attr_id: AAttrId,
}
impl<'a> FullSideEffect<'a> {
    pub(in crate::sol::api) fn new(
        sol: &'a SolarSystem,
        key: UItemKey,
        effect_id: AEffectId,
        chance_attr_id: AAttrId,
    ) -> Self {
        Self {
            sol,
            key,
            effect_id,
            chance_attr_id,
        }
    }
    pub fn get_effect_id(&self) -> EffectId {
        self.effect_id.into()
    }
}

/// Side effect which has full functionality available.
pub struct FullSideEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
    pub(in crate::sol::api) effect_id: AEffectId,
    pub(in crate::sol::api) chance_attr_id: AAttrId,
}
impl<'a> FullSideEffectMut<'a> {
    pub(in crate::sol::api) fn new(
        sol: &'a mut SolarSystem,
        key: UItemKey,
        effect_id: AEffectId,
        chance_attr_id: AAttrId,
    ) -> Self {
        Self {
            sol,
            key,
            effect_id,
            chance_attr_id,
        }
    }
    pub fn get_effect_id(&self) -> EffectId {
        self.effect_id.into()
    }
}

/// A non-side-effect effect, with limited functionality. Exists to expose and let manipulate side
/// effect data on data sources which don't have this effect defined as a side effect.
pub struct StubSideEffect<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
    pub(in crate::sol::api) effect_id: AEffectId,
}
impl<'a> StubSideEffect<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey, effect_id: AEffectId) -> Self {
        Self { sol, key, effect_id }
    }
    pub fn get_effect_id(&self) -> EffectId {
        self.effect_id.into()
    }
}

/// A non-side-effect effect, with limited functionality. Exists to expose and let manipulate side
/// effect data on data sources which don't have this effect defined as a side effect.
pub struct StubSideEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
    pub(in crate::sol::api) effect_id: AEffectId,
}
impl<'a> StubSideEffectMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey, effect_id: AEffectId) -> Self {
        Self { sol, key, effect_id }
    }
    pub fn get_effect_id(&self) -> EffectId {
        self.effect_id.into()
    }
}
