use crate::{def::Id, ed::EItemCatId};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AItemCatId(Id);
impl AItemCatId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
impl const From<EItemCatId> for AItemCatId {
    fn from(cat_eid: EItemCatId) -> Self {
        Self::new(cat_eid.into_inner())
    }
}
