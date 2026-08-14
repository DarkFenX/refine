use crate::{
    FitId,
    val::{SolValInfo, ValInfoMode, ValInfoModes, ValOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ValidateSolCmd {
    options: ValOptions = ValOptions { default: true, .. },
    fit_ids: Vec<FitId> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValidateSolCmd {
    pub fn new(options: ValOptions) -> Self {
        Self {
            options,
            fit_ids: Vec::new(),
        }
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit_ids.clear();
        self.fit_ids.extend(fit_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValidateSolCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem, modes: ValInfoModes) -> SolValInfo {
        let core_options = rc::val::ValOptionsSol {
            fit_ids: self.fit_ids,
            options: self.options,
        };
        match modes.validation {
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
