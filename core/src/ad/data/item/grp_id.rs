use crate::{ed::EItemGrpId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AItemGrpId(i32);
impl AItemGrpId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemGrpId {
    pub(in crate::ad) const fn from_eid(grp_eid: EItemGrpId) -> Self {
        Self(grp_eid.into_i32())
    }
    pub(crate) fn from_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
}
