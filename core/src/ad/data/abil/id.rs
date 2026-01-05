use crate::ed::EAbilId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AAbilId(i32);
impl AAbilId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
impl From<EAbilId> for AAbilId {
    fn from(abil_eid: EAbilId) -> Self {
        Self::new(abil_eid.into_inner())
    }
}
