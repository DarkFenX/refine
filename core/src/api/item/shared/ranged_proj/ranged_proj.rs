use crate::{def::ItemId, misc::ProjRange, sol::SolarSystem, ud::UItemKey};

/// Projection which allows to set range.
pub struct RangedProj<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) projector_key: UItemKey,
    pub(in crate::api) projectee_key: UItemKey,
}
impl<'a> RangedProj<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, projector_key: UItemKey, projectee_key: UItemKey) -> Self {
        Self {
            sol,
            projector_key,
            projectee_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.id_by_key(self.projectee_key)
    }
    pub fn get_range(&self) -> Option<ProjRange> {
        get_range(self.sol, self.projector_key, &self.projectee_key)
    }
}

/// Projection which allows to set range.
pub struct RangedProjMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) projector_key: UItemKey,
    pub(in crate::api) projectee_key: UItemKey,
}
impl<'a> RangedProjMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, projector_key: UItemKey, projectee_key: UItemKey) -> Self {
        Self {
            sol,
            projector_key,
            projectee_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.id_by_key(self.projectee_key)
    }
    pub fn get_range(&self) -> Option<ProjRange> {
        get_range(self.sol, self.projector_key, &self.projectee_key)
    }
}

fn get_range(sol: &SolarSystem, projector_key: UItemKey, projectee_key: &UItemKey) -> Option<ProjRange> {
    sol.u_data
        .items
        .get(projector_key)
        .get_projs()
        .unwrap()
        .get(projectee_key)
        .unwrap()
        .map(Into::into)
}
