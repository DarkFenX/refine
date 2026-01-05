use crate::{ed::EItemGrpId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AItemGrpId(i32);
impl AItemGrpId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
    pub(crate) fn new_f64_rounded(id: f64) -> Self {
        Self::new(round_f64_to_i32(id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl const From<EItemGrpId> for AItemGrpId {
    fn from(grp_eid: EItemGrpId) -> Self {
        Self::new(grp_eid.into_inner())
    }
}
