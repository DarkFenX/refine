use crate::ed::EItemCatId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AItemCatId(i32);
impl AItemCatId {
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
impl AItemCatId {
    pub(in crate::ad) const fn from_eid(cat_eid: EItemCatId) -> Self {
        Self(cat_eid.into_i32())
    }
}
