use crate::def::DefId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EItemListId(DefId);
impl EItemListId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
