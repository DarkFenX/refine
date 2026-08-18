use crate::{
    FitId, FitIdBr, ItemId, ItemIdBr,
    val::{FitValInfo, ValInfoMode, ValOptions},
};

// Core commands
#[derive(Clone, Default)]
pub struct FitValCmd {
    options: ValOptions<ItemId>,
    shared: FitValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct FitValCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: FitValCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
struct FitValCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValInfoMode,
}

// Extra context commands
#[derive(Clone)]
pub struct FitValCmdCtxFit {
    fit_id: FitId,
    core: FitValCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct FitValCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: FitValCmdBr,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemId>) -> Self {
        self.options = options;
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
        self
    }
}

impl FitValCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions<ItemIdBr>) -> Self {
        self.options = options;
        self
    }
    pub fn with_info_mode(mut self, info_mode: ValInfoMode) -> Self {
        self.shared.info_mode = info_mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmdBr {
    pub(in crate::val) fn into_ctx_item_br(self, fit_id: impl Into<FitIdBr>) -> FitValCmdCtxFitBr {
        FitValCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitValInfo {
        match self.shared.info_mode {
            ValInfoMode::Simple => FitValInfo {
                passed: core_fit.validate_fast(&self.options),
                details: None,
            },
            ValInfoMode::Detailed => {
                let details = core_fit.validate_verbose(&self.options);
                FitValInfo {
                    passed: details.all_passed(),
                    details: Some(details),
                }
            }
        }
    }
}
