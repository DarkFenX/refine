use crate::{ItemId, ProjRange, SolarSystem, ud::UItemId};

/// Projection which allows to set range.
pub struct RangedProj<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) projector_uid: UItemId,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'s> RangedProj<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            sol,
            projector_uid,
            projectee_uid,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.ext_id_by_int_id(self.projectee_uid)
    }
    pub fn get_range(&self) -> ProjRange {
        get_range(self.sol, self.projector_uid, &self.projectee_uid)
    }
}

/// Projection which allows to set range.
pub struct RangedProjMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) projector_uid: UItemId,
    pub(in crate::api) projectee_uid: UItemId,
}
impl<'s> RangedProjMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            sol,
            projector_uid,
            projectee_uid,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.ext_id_by_int_id(self.projectee_uid)
    }
    pub fn get_range(&self) -> ProjRange {
        get_range(self.sol, self.projector_uid, &self.projectee_uid)
    }
}

fn get_range(sol: &SolarSystem, projector_uid: UItemId, projectee_uid: &UItemId) -> ProjRange {
    // - unwrap #1 - projection itself is fetchable only for item with projection data on them
    // - unwrap #2 - projection itself is fetchable only for projections which exist
    // - unwrap #3 - ranged projection should be exposed only for items which have ranged projection,
    //   i.e. put some value into projection container
    let u_proj = sol
        .u_data
        .items
        .get(projector_uid)
        .get_projs()
        .unwrap()
        .get(projectee_uid)
        .unwrap()
        .unwrap();
    ProjRange::from_u_proj_data(u_proj)
}
