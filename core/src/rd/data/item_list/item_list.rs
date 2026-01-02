use crate::{
    ad::{AItemId, AItemList, AItemListId},
    rd::RItemListId,
    util::RSet,
};

// Represents an item list.
//
// Item lists in their rendered form carry just that, an item list.
pub(crate) struct RItemList {
    pub(crate) key: RItemListId,
    pub(crate) id: AItemListId,
    pub(crate) item_ids: RSet<AItemId>,
}
impl RItemList {
    pub(in crate::rd) fn from_a_item_list(item_list_key: RItemListId, a_item_list: &AItemList) -> Self {
        Self {
            key: item_list_key,
            id: a_item_list.id,
            item_ids: a_item_list.item_ids.clone(),
        }
    }
}
