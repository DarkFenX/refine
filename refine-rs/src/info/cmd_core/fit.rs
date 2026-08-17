use crate::{
    FitInfo, FitInfoMode, ItemId, ItemIdBr, ItemInfoMode,
    info::{InfoModes, InfoModesCompact},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModes<ItemInfoMode, ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModesCompact<ItemInfoMode, ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
struct FitInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit = mode;
        self
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

impl FitInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit(mut self, mode: FitInfoMode) -> Self {
        self.shared.fit = mode;
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitInfo {
        FitInfo::from_core(core_fit, &InfoModes::from_simple(self.shared.fit), &self.item)
    }
}
