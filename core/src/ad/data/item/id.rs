use crate::{
    def::{Id, Value},
    ed::EItemId,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AItemId(Id);
impl AItemId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub(crate) fn new_of64(id: Value) -> Self {
        Self(id.into_inner().clamp(Id::MIN as f64, Id::MAX as f64).round() as Id)
    }
    pub fn into_inner(self) -> Id {
        self.0
    }
}
impl const From<EItemId> for AItemId {
    fn from(item_eid: EItemId) -> Self {
        Self::new(item_eid.into_inner())
    }
}
