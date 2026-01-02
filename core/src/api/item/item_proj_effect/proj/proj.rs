use crate::{def::ItemId, sol::SolarSystem, ud::UItemId};

/// Projection which does not allow to set range.
pub struct Proj<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) projectee_key: UItemId,
}
impl<'a> Proj<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, projectee_key: UItemId) -> Self {
        Self { sol, projectee_key }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.ext_id_by_int_id(self.projectee_key)
    }
}

/// Projection which does not allow to set range.
pub struct ProjMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) projector_key: UItemId,
    pub(in crate::api) projectee_key: UItemId,
}
impl<'a> ProjMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, projector_key: UItemId, projectee_key: UItemId) -> Self {
        Self {
            sol,
            projector_key,
            projectee_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.ext_id_by_int_id(self.projectee_key)
    }
}
