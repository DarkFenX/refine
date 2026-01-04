use crate::def::DefId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EItemId(DefId);
impl EItemId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
