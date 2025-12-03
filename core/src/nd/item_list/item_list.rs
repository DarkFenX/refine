use crate::{
    ad::{AItem, AItemListId},
    ed::EItemListId,
};

pub(crate) type NItemListFilter = fn(&AItem) -> bool;

pub(crate) struct NItemList {
    // EVE data item list ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<EItemListId>,
    // Adapted data item list ID
    pub(crate) aid: AItemListId,
    // Function which controls which items will be in the list
    pub(crate) adg_item_filter_fn: Option<NItemListFilter> = None,
}
