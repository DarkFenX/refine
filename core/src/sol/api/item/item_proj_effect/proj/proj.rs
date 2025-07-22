use crate::{def::ItemId, sol::SolarSystem, uad::UadItemKey};

/// Projection which does not allow to set range.
pub struct Proj<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) projectee_key: UadItemKey,
}
impl<'a> Proj<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, projectee_key: UadItemKey) -> Self {
        Self { sol, projectee_key }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.uad.items.id_by_key(self.projectee_key)
    }
}

/// Projection which does not allow to set range.
pub struct ProjMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) projector_key: UadItemKey,
    pub(in crate::sol::api) projectee_key: UadItemKey,
}
impl<'a> ProjMut<'a> {
    pub(in crate::sol::api) fn new(
        sol: &'a mut SolarSystem,
        projector_key: UadItemKey,
        projectee_key: UadItemKey,
    ) -> Self {
        Self {
            sol,
            projector_key,
            projectee_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.uad.items.id_by_key(self.projectee_key)
    }
}
