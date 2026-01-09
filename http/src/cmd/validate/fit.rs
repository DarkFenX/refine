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
        let core_options = (&self.validation_options).into();
        let mut primary_fit = get_primary_fit(core_sol, fit_id)?;
        Ok(match valid_mode {
            HValidInfoMode::Simple => primary_fit.validate_fast(&core_options).into(),
            HValidInfoMode::Detailed => (&primary_fit.validate_verbose(&core_options)).into(),
        })
    }
}
