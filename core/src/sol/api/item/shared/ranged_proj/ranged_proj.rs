use crate::{def::ItemId, misc::ProjRangeInfo, sol::SolarSystem, uad::UadItemKey};

/// Projection which allows to set range.
pub struct RangedProj<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) projector_key: UadItemKey,
    pub(in crate::sol::api) projectee_key: UadItemKey,
}
impl<'a> RangedProj<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, projector_key: UadItemKey, projectee_key: UadItemKey) -> Self {
        Self {
            sol,
            projector_key,
            projectee_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.uad.items.id_by_key(self.projectee_key)
    }
    pub fn get_range(&self) -> Option<ProjRangeInfo> {
        get_range(self.sol, self.projector_key, &self.projectee_key)
    }
}

/// Projection which allows to set range.
pub struct RangedProjMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) projector_key: UadItemKey,
    pub(in crate::sol::api) projectee_key: UadItemKey,
}
impl<'a> RangedProjMut<'a> {
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
    pub fn get_range(&self) -> Option<ProjRangeInfo> {
        get_range(self.sol, self.projector_key, &self.projectee_key)
    }
}

fn get_range(sol: &SolarSystem, projector_key: UadItemKey, projectee_key: &UadItemKey) -> Option<ProjRangeInfo> {
    sol.uad
        .items
        .get(projector_key)
        .get_projs()
        .unwrap()
        .get(projectee_key)
        .unwrap()
        .map(|v| v.into())
}
