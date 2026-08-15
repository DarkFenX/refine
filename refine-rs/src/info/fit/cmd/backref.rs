use crate::{
    CtlCmdResps, FitInfo, FitInfoMode, ItemIdBackref, ItemInfoMode, ItemInfoModesBackref,
    info::{FitInfoModesInt, ItemInfoModesInt},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitInfoCmdBackref {
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoMode = FitInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    item: ItemInfoModesBackref = ItemInfoModesBackref::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmdBackref {
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
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfoCmdBackref {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut, ctl_cmd_resps: &CtlCmdResps) -> FitInfo {
        FitInfo::from_core(
            core_fit,
            &FitInfoModesInt::from_pub_mode(self.fit),
            &ItemInfoModesInt::from_pub_modes_backref(self.item, ctl_cmd_resps),
        )
    }
}
