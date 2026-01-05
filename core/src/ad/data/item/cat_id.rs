use crate::ed::EItemCatId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AItemCatId(i32);
impl AItemCatId {
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
impl const From<EItemCatId> for AItemCatId {
    fn from(cat_eid: EItemCatId) -> Self {
        Self::new(cat_eid.into_inner())
    }
}
