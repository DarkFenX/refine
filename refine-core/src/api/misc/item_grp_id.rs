use crate::ad::AItemGrpId;

/// Fighter ability ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct ItemGrpId(i32);
impl ItemGrpId {
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
impl ItemGrpId {
    pub(crate) fn from_aid(item_grp_aid: AItemGrpId) -> Self {
        Self(item_grp_aid.into_i32())
    }
}
