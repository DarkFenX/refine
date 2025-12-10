use crate::ad::{ACustomItemListId, AEveItemListId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AItemListId {
    Eve(AEveItemListId),
    Custom(ACustomItemListId),
}
impl AItemListId {
    pub(crate) fn dc_eve(&self) -> Option<AEveItemListId> {
        match self {
            Self::Eve(eve_item_list_id) => Some(*eve_item_list_id),
            _ => None,
        }
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
