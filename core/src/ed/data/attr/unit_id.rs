use crate::def::Id;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EAttrUnitId(Id);
impl EAttrUnitId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
