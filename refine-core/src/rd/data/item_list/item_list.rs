use crate::{
    ad::{AItemId, AItemList},
    util::RSet,
};

// Represents an item list.
//
// Item lists in their rendered form carry just that, an item list.
pub(crate) struct RItemList {
    pub(crate) item_aids: RSet<AItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemList {
    pub(in crate::rd::data) fn from_a_item_list(a_item_list: &AItemList) -> Self {
        Self {
            item_aids: a_item_list.item_ids.iter().copied().collect(),
        }
    }
}
