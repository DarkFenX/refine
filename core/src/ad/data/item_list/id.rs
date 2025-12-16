use crate::ad::{ACustomItemListId, AEveItemListId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AItemListId {
    Eve(AEveItemListId),
    Custom(ACustomItemListId),
}
impl std::fmt::Display for AItemListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "e{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}
