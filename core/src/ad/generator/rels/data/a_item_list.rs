use crate::{ad::AItemListId, ed::EItemListId};

impl AItemListId {
    pub(super) fn dc_eve(&self) -> Option<EItemListId> {
        match self {
            Self::Eve(eve_item_list_aid) => Some(EItemListId::new(eve_item_list_aid.into_inner())),
            _ => None,
        }
    }
}
