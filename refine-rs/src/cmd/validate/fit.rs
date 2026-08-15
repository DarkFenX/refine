use crate::val::{FitValInfo, ValInfoArgs, ValInfoMode, ValOptions};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ValidateFitCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: ValOptions = ValOptions { .. },
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
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut, info_args: ValInfoArgs) -> FitValInfo {
        match info_args.validation {
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
