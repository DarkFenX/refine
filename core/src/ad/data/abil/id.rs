use crate::ed::EAbilId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AAbilId(i32);
impl AAbilId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl const From<EAbilId> for AAbilId {
    fn from(abil_eid: EAbilId) -> Self {
        Self::new(abil_eid.into_inner())
    }
}
