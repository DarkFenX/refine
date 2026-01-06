use crate::ed::EEffectCatId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AEffectCatId(i32);
impl AEffectCatId {
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
impl AEffectCatId {
    pub(in crate::ad) const fn from_eid(effect_cat_eid: EEffectCatId) -> Self {
        Self(effect_cat_eid.into_i32())
    }
}
