use crate::{
    def::{Id, Value},
    ed::EItemListId,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AItemListId {
    Eve(AEveItemListId),
    Custom(ACustomItemListId),
}
impl const From<EItemListId> for AItemListId {
    fn from(item_list_eid: EItemListId) -> Self {
        Self::Eve(AEveItemListId(item_list_eid.into_inner()))
    }
}
impl std::fmt::Display for AItemListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AEveItemListId(Id);
impl AEveItemListId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub(crate) fn new_of64(id: Value) -> Self {
        Self(id.into_inner().clamp(Id::MIN as f64, Id::MAX as f64).round() as Id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomItemListId(Id);
impl ACustomItemListId {
    pub const fn new(id: Id) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> Id {
        self.0
    }
}
