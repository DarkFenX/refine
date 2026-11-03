use crate::{ItemId, SolarSystem, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Projection which does not allow to set range.
pub struct Proj<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) projector_uid: UItemId,
    pub(super) projectee_uid: UItemId,
}
impl<'s> Proj<'s> {
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.u_data.items.ext_id_by_int_id(self.projectee_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Proj<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            sol,
            projector_uid,
            projectee_uid,
        }
    }
}
