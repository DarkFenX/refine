use crate::{def::Id, ed::EItemGrpId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AItemGrpId(Id);
impl AItemGrpId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
impl const From<EItemGrpId> for AItemGrpId {
    fn from(grp_eid: EItemGrpId) -> Self {
        Self::new(grp_eid.into_inner())
    }
}
