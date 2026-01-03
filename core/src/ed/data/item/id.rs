use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EItemId(Id);
impl EItemId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
