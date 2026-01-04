use crate::{def::DefId, ed::EAbilId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AAbilId(DefId);
impl AAbilId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
impl From<EAbilId> for AAbilId {
    fn from(abil_eid: EAbilId) -> Self {
        Self::new(abil_eid.into_inner())
    }
}
