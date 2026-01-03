use crate::{def::Id, ed::EEffectCatId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AEffectCatId(Id);
impl AEffectCatId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
impl const From<EEffectCatId> for AEffectCatId {
    fn from(effect_cat_eid: EEffectCatId) -> Self {
        Self::new(effect_cat_eid.into_inner())
    }
}
