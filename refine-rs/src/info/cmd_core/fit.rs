use crate::{
    CtlCmdResps, FitInfo, FitInfoMode, ItemId, ItemIdBr, ItemInfoMode,
    info::{InfoModes, InfoModesCompact},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoMode,
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModesCompact<ItemInfoMode, ItemId>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoMode,
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModesCompact<ItemInfoMode, ItemIdBr>,
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

impl FitInfoCmdBr {
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
        FitInfo::from_core(
            core_fit,
            &InfoModes::from_simple(self.fit),
            &InfoModes::from_compact(self.item),
        )
    }
}

impl FitInfoCmdBr {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut, ctl_cmd_resps: &CtlCmdResps) -> FitInfo {
        FitInfo::from_core(
            core_fit,
            &InfoModes::from_simple(self.fit),
            &InfoModes::from_compact_br(self.item, ctl_cmd_resps),
        )
    }
}
