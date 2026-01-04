use crate::{def::DefId, ed::EEffectCatId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AEffectCatId(DefId);
impl AEffectCatId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
impl const From<EEffectCatId> for AEffectCatId {
    fn from(effect_cat_eid: EEffectCatId) -> Self {
        Self::new(effect_cat_eid.into_inner())
    }
}
