use crate::ad::{AItem, AItemListId};

pub(crate) type NItemListFilter = fn(&AItem) -> bool;

pub(crate) struct NItemList {
    // Adapted data item list ID
    pub(crate) aid: AItemListId,
    // Function which controls which items will be in the list
    pub(crate) adg_item_filter_fn: Option<NItemListFilter> = None,
}
