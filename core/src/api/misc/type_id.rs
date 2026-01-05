use crate::ad::AItemId;

/// Item type ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct ItemTypeId(i32);
impl ItemTypeId {
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
impl From<AItemId> for ItemTypeId {
    fn from(item_aid: AItemId) -> Self {
        Self::new(item_aid.into_inner())
    }
}
impl From<&AItemId> for ItemTypeId {
    fn from(item_aid: &AItemId) -> Self {
        Self::new(item_aid.into_inner())
    }
}
impl From<ItemTypeId> for AItemId {
    fn from(item_type_id: ItemTypeId) -> Self {
        Self::new(item_type_id.into_inner())
    }
}
impl From<&ItemTypeId> for AItemId {
    fn from(item_type_id: &ItemTypeId) -> Self {
        Self::new(item_type_id.into_inner())
    }
}
