use crate::{def::DefId, ed::EItemGrpId, util::f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AItemGrpId(DefId);
impl AItemGrpId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64(id: f64) -> Self {
        Self::new(f64_to_i32(id))
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
impl const From<EItemGrpId> for AItemGrpId {
    fn from(grp_eid: EItemGrpId) -> Self {
        Self::new(grp_eid.into_inner())
    }
}
