use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EItemListId(Id);
impl EItemListId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
