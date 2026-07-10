use crate::ad::{AItem, AItemId};

pub(crate) type NItemUpdater = fn(&mut AItem);

pub(crate) struct NItem {
    // Adapted data item ID
    pub(crate) aid: AItemId,
    // Fields related to adapted data generation
    pub(crate) adg_update_item_fn: Option<NItemUpdater> = None,
}
