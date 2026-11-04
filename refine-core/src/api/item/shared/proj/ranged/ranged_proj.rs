use crate::{ItemId, ProjRange, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Projection which allows to set range.
pub struct RangedProj<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) projector_uid: UItemId,
    pub(super) projectee_uid: UItemId,
}
impl<'s> RangedProj<'s> {
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.core.u_data.items.ext_id_by_int_id(self.projectee_uid)
    }
    pub fn get_range(&self) -> ProjRange {
        let projectee_uid = &self.projectee_uid;
        // - unwrap #1 - projection itself is fetchable only for item with projection data on them
        // - unwrap #2 - projection itself is fetchable only for projections which exist
        // - unwrap #3 - ranged projection should be exposed only for items which have ranged
        //   projection, i.e. put some value into projection container
        let u_proj = self
            .core
            .u_data
            .items
            .get(self.projector_uid)
            .get_projs()
            .unwrap()
            .get(projectee_uid)
            .unwrap()
            .unwrap();
        ProjRange::from_u_proj_data(u_proj)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> RangedProj<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, projector_uid: UItemId, projectee_uid: UItemId) -> Self {
        Self {
            core,
            projector_uid,
            projectee_uid,
        }
    }
}
