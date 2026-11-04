use crate::{ItemId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Projection which does not allow to set range.
pub struct Proj<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) projector_uid: UItemId,
    pub(super) projectee_uid: UItemId,
}
impl<'s> Proj<'s> {
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.core.u_data.items.ext_id_by_int_id(self.projectee_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Proj<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            core,
            projector_uid,
            projectee_uid,
        }
    }
}
