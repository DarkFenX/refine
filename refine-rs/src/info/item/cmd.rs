use crate::{ItemId, ItemInfo, ItemInfoMode, ItemInfoModes, info::ItemInfoModesInt};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item: ItemInfoModes = ItemInfoModes::default(),
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
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemInfo {
        ItemInfo::from_core(core_item, &ItemInfoModesInt::from_pub_modes_regular(self.item))
    }
}
