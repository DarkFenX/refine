#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct EItemListId(i32);
impl EItemListId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}
