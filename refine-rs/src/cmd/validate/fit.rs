use crate::val::{FitValInfo, ValInfoMode, ValOptions};

pub struct ValidateFitCmd {
    options: ValOptions,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValidateFitCmd {
    pub fn new(options: ValOptions) -> Self {
        Self { options }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValidateFitCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut, val_mode: ValInfoMode) -> FitValInfo {
        match val_mode {
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
