use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EEffectId(Id);
impl EEffectId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
