use crate::ad::AAbilId;

/// Fighter ability ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AbilId(i32);
impl AbilId {
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
impl From<AAbilId> for AbilId {
    fn from(item_aid: AAbilId) -> Self {
        Self::new(item_aid.into_inner())
    }
}
impl From<&AAbilId> for AbilId {
    fn from(item_aid: &AAbilId) -> Self {
        Self::new(item_aid.into_inner())
    }
}
impl From<AbilId> for AAbilId {
    fn from(abil_id: AbilId) -> Self {
        Self::new(abil_id.into_inner())
    }
}
impl From<&AbilId> for AAbilId {
    fn from(abil_id: &AbilId) -> Self {
        Self::new(abil_id.into_inner())
    }
}
