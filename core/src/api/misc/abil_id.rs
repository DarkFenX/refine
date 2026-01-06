use crate::ad::AAbilId;

/// Fighter ability ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AbilId(i32);
impl AbilId {
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
impl AbilId {
    pub(in crate::api) fn from_aid(abil_aid: AAbilId) -> Self {
        Self(abil_aid.into_i32())
    }
    pub(in crate::api) fn into_aid(self) -> AAbilId {
        AAbilId::from_i32(self.0)
    }
}
