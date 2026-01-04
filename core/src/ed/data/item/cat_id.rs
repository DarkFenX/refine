use crate::def::DefId;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EItemCatId(DefId);
impl EItemCatId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
