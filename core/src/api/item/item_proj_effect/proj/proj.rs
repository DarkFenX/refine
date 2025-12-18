use crate::{def::ItemId, sol::SolarSystem, ud::UItemKey};

/// Projection which does not allow to set range.
pub struct Proj<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) projectee_key: UItemKey,
}
impl<'a> Proj<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, projectee_key: UItemKey) -> Self {
        Self { sol, projectee_key }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.id_by_key(self.projectee_key)
    }
}

/// Projection which does not allow to set range.
pub struct ProjMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) projector_key: UItemKey,
    pub(in crate::api) projectee_key: UItemKey,
}
impl<'a> ProjMut<'a> {
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
}
