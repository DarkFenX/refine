use crate::{ad::AItemListId, ed::EItemListId};

impl AItemListId {
    pub(super) fn dc_eve(&self) -> Option<EItemListId> {
        match self {
            Self::Eve(eve_item_list_id) => Some(*eve_item_list_id),
            _ => None,
        }
    }
}
