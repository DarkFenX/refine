use crate::def::DefId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EAttrUnitId(DefId);
impl EAttrUnitId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
