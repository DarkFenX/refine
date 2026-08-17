use crate::{ItemId, ItemInfo, ItemInfoMode, info::InfoModes};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModes<ItemInfoMode, ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        for item_id in item_ids {
            self.item.overrides.insert(item_id, mode);
        }
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemInfo {
        ItemInfo::from_core(core_item, &self.item)
    }
}
