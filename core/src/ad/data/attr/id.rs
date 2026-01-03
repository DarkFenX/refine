use crate::{def::Id, ed::EAttrId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}
impl const From<EAttrId> for AAttrId {
    fn from(attr_eid: EAttrId) -> Self {
        Self::Eve(AEveAttrId(attr_eid.into_inner()))
    }
}
impl std::fmt::Display for AAttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AEveAttrId(Id);
impl AEveAttrId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomAttrId(Id);
impl ACustomAttrId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
