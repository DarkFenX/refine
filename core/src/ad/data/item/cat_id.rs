use crate::{def::DefId, ed::EItemCatId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AItemCatId(DefId);
impl AItemCatId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
impl const From<EItemCatId> for AItemCatId {
    fn from(cat_eid: EItemCatId) -> Self {
        Self::new(cat_eid.into_inner())
    }
}
