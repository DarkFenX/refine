use crate::ed::EAbilId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AAbilId(i32);
impl AAbilId {
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
impl AAbilId {
    pub(in crate::ad) const fn from_eid(abil_eid: EAbilId) -> Self {
        Self(abil_eid.into_i32())
    }
}
