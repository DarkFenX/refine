use crate::{ad::AAbilId, sol::SolarSystem, ud::UItemKey};

/// Fighter ability.
pub struct Ability<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) item_key: UItemKey,
    pub(in crate::sol::api) abil_id: AAbilId,
}
impl<'a> Ability<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, item_key: UItemKey, abil_id: AAbilId) -> Self {
        Self { sol, item_key, abil_id }
    }
}

/// Fighter ability which allows changing its state.
pub struct AbilityMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) item_key: UItemKey,
    pub(in crate::sol::api) abil_id: AAbilId,
}
impl<'a> AbilityMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, item_key: UItemKey, abil_id: AAbilId) -> Self {
        Self { sol, item_key, abil_id }
    }
}
