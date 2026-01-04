use crate::def::DefId;

#[derive(Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub struct EEffectCatId(DefId);
impl EEffectCatId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
