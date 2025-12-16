use crate::{
    ad::{
        AItemListId,
        generator::rels::{Fk, KeyPart, Pk},
    },
    ed::{EItemList, EItemListId},
};

impl Pk for EItemList {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

// No actual implementation for item lists, since we do not plan to restore anything linked from
// item lists, and item lists themselves will be cleaned up during conversion
impl Fk for EItemList {}

impl AItemListId {
    pub(super) fn dc_eve(&self) -> Option<EItemListId> {
        match self {
            Self::Eve(eve_item_list_id) => Some(*eve_item_list_id),
            _ => None,
        }
    }
}
