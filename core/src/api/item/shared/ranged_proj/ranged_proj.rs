use crate::{
    api::ProjRange,
    sol::SolarSystem,
    ud::{ItemId, UItemId},
};

/// Projection which allows to set range.
pub struct RangedProj<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) projector_uid: UItemId,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'a> RangedProj<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            sol,
            projector_uid,
            projectee_uid,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.xid_by_iid(self.projectee_uid)
    }
    pub fn get_range(&self) -> Option<ProjRange> {
        get_range(self.sol, self.projector_uid, &self.projectee_uid)
    }
}

/// Projection which allows to set range.
pub struct RangedProjMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) projector_uid: UItemId,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'a> RangedProjMut<'a> {
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
    pub fn get_range(&self) -> Option<ProjRange> {
        get_range(self.sol, self.projector_uid, &self.projectee_uid)
    }
}

fn get_range(sol: &SolarSystem, projector_uid: UItemId, projectee_uid: &UItemId) -> Option<ProjRange> {
    sol.u_data
        .items
        .get(projector_uid)
        .get_projs()
        .unwrap()
        .get(projectee_uid)
        .unwrap()
        .map(ProjRange::from_u_proj_data)
}
