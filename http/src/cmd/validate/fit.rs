use serde::Deserialize;

use crate::{
    cmd::shared::{HValOptions, get_primary_fit},
    info::{HFitValResult, HValidInfoMode},
    util::HExecError,
};

#[derive(Deserialize, Default)]
#[serde(transparent)]
pub(crate) struct HValidateFitCmd {
    validation_options: HValOptions,
}
impl HValidateFitCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
        valid_mode: HValidInfoMode,
    ) -> Result<HFitValResult, HExecError> {
        let core_options = self.validation_options.to_core();
        let mut primary_fit = get_primary_fit(core_sol, fit_id)?;
        Ok(match valid_mode {
            HValidInfoMode::Simple => HFitValResult::from_core_simple(primary_fit.validate_fast(&core_options)),
            HValidInfoMode::Detailed => HFitValResult::from_core_detailed(primary_fit.validate_verbose(&core_options)),
        })
    }
}
