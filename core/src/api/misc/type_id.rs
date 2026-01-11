use crate::ad::AItemId;

/// Item type ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct ItemTypeId(i32);
impl ItemTypeId {
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
impl ItemTypeId {
    pub(crate) fn from_aid(type_aid: AItemId) -> Self {
        Self(type_aid.into_i32())
    }
    pub(in crate::api) fn into_aid(self) -> AItemId {
        AItemId::from_i32(self.0)
    }
}
