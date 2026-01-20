use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValNotLoadedItemFail {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    item_ids: Vec<rc::ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValNotLoadedItemFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValNotLoadedItemFail) -> Self {
        Self {
            item_ids: core_val_fail.item_ids,
        }
    }
}
