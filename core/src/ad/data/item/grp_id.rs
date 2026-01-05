use crate::{ed::EItemGrpId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AItemGrpId(i32);
impl AItemGrpId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64_rounded(id: f64) -> Self {
        Self::new(round_f64_to_i32(id))
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
impl const From<EItemGrpId> for AItemGrpId {
    fn from(grp_eid: EItemGrpId) -> Self {
        Self::new(grp_eid.into_inner())
    }
}
