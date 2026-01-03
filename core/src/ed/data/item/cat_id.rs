use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EItemCatId(Id);
impl EItemCatId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
