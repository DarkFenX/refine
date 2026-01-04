use crate::{def::DefId, ed::EItemId, util::f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AItemId(DefId);
impl AItemId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64(id: f64) -> Self {
        Self(f64_to_i32(id))
    }
    pub fn into_inner(self) -> DefId {
        self.0
    }
}
impl const From<EItemId> for AItemId {
    fn from(item_eid: EItemId) -> Self {
        Self::new(item_eid.into_inner())
    }
}
