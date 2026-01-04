use crate::{def::DefId, ed::EItemListId, util::f64_to_i32};

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
pub struct AEveItemListId(DefId);
impl AEveItemListId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub(crate) fn new_f64(id: f64) -> Self {
        Self(f64_to_i32(id))
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomItemListId(DefId);
impl ACustomItemListId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
