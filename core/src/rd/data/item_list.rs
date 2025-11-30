use crate::{
    ad::{AItemId, AItemList, AItemListId},
    util::{GetId, Named, RSet},
};

// Represents an item list.
//
// Item lists in their rendered form carry just that, an item list.
pub(crate) struct RItemList {
    a_item_list: AItemList,
}
impl RItemList {
    pub(crate) fn new(a_item_list: AItemList) -> Self {
        Self { a_item_list }
    }
    pub(crate) fn get_item_ids(&self) -> &RSet<AItemId> {
        &self.a_item_list.item_ids
    }
}
impl GetId<AItemListId> for RItemList {
    fn get_id(&self) -> AItemListId {
        self.a_item_list.id
    }
}
impl Named for RItemList {
    fn get_name() -> &'static str {
        "RItemList"
    }
}
