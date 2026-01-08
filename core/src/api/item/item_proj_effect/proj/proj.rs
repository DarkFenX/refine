use crate::{
    sol::SolarSystem,
    ud::{ItemId, UItemId},
};

/// Projection which does not allow to set range.
pub struct Proj<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'a> Proj<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, projectee_uid: UItemId) -> Self {
        Self { sol, projectee_uid }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.xid_by_iid(self.projectee_uid)
    }
}

/// Projection which does not allow to set range.
pub struct ProjMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) projector_uid: UItemId,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'a> ProjMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            sol,
            projector_uid,
            projectee_uid,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.xid_by_iid(self.projectee_uid)
    }
}
