use crate::{
    ItemId,
    val::{FitValInfo, ValInfoMode, ValOptions},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct FitValCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions<ItemId>,
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValInfoMode,
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
        self.info_mode = info_mode;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitValCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitValInfo {
        match self.info_mode {
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
