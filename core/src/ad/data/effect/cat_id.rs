use crate::ed::EEffectCatId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AEffectCatId(i32);
impl AEffectCatId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
impl const From<EEffectCatId> for AEffectCatId {
    fn from(effect_cat_eid: EEffectCatId) -> Self {
        Self::new(effect_cat_eid.into_inner())
    }
}
