use crate::{
    ad::{AItemId, AItemList, AItemListId},
    rd::RItemListId,
    util::RSet,
};

// Represents an item list.
//
// Item lists in their rendered form carry just that, an item list.
pub(crate) struct RItemList {
    pub(crate) aid: AItemListId,
    pub(crate) rid: RItemListId,
    pub(crate) item_aids: RSet<AItemId>,
}
impl RItemList {
    pub(in crate::rd) fn from_a_item_list(item_list_rid: RItemListId, a_item_list: &AItemList) -> Self {
        Self {
            aid: a_item_list.id,
            rid: item_list_rid,
            item_aids: a_item_list.item_ids.iter().copied().collect(),
        }
    }
}
