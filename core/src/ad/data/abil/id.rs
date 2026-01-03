use crate::{def::Id, ed::EAbilId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AAbilId(Id);
impl AAbilId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
impl From<EAbilId> for AAbilId {
    fn from(abil_eid: EAbilId) -> Self {
        Self::new(abil_eid.into_inner())
    }
}
