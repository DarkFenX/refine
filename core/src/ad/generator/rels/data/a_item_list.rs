use crate::{ad::AItemListId, ed::EItemListId};

impl AItemListId {
    pub(super) fn dc_eve(&self) -> Option<EItemListId> {
        match self {
            Self::Eve(eve_item_list_aid) => Some(EItemListId::from_i32(eve_item_list_aid.into_i32())),
            _ => None,
        }
    }
}
