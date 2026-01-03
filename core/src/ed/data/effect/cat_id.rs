use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub struct EEffectCatId(Id);
impl EEffectCatId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
