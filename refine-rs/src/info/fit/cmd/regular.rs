use crate::{
    FitInfo, FitInfoMode, ItemId, ItemInfoMode, ItemInfoModes,
    info::{FitInfoModesInt, ItemInfoModesInt},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    item: ItemInfoModes = ItemInfoModes::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.fit = mode;
        self
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
impl FitInfoCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitInfo {
        let fit_info_modes = FitInfoModesInt::from_pub_mode(self.fit);
        let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(self.item);
        FitInfo::from_core(core_fit, &fit_info_modes, &item_info_modes)
    }
}
