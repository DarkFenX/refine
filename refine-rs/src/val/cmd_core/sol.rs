use crate::{
    FitId,
    val::{SolValInfo, ValInfoMode, ValOptions},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct SolValCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    options: ValOptions,
    #[cfg_attr(feature = "serde", serde(default))]
    fit_ids: Vec<FitId>,
    #[cfg_attr(feature = "serde", serde(default))]
    info_mode: ValInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolValCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_options(mut self, options: ValOptions) -> Self {
        self.options = options;
        self
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_ids.extend(fit_ids);
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
impl SolValCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> SolValInfo {
        let core_options = rc::val::ValOptionsSol {
            fit_ids: self.fit_ids,
            options: self.options,
        };
        match self.info_mode {
            ValInfoMode::Simple => SolValInfo {
                passed: core_sol.validate_fast(&core_options),
                details: None,
            },
            ValInfoMode::Detailed => {
                let details = core_sol.validate_verbose(&core_options);
                SolValInfo {
                    passed: details.all_passed(),
                    details: Some(details),
                }
            }
        }
    }
}
