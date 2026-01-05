#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EItemGrpId(i32);
impl EItemGrpId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
