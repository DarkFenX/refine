use crate::{
    cmd::shared::{HValOptions, get_primary_fit},
    info::{HValidInfo, HValidInfoMode},
    util::HExecError,
};

#[derive(serde::Deserialize, Default)]
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
    ) -> Result<HValidInfo, HExecError> {
        let core_options = self.validation_options.to_core_val_options(core_sol);
        let mut primary_fit = get_primary_fit(core_sol, fit_id)?;
        Ok(match valid_mode {
            HValidInfoMode::Simple => primary_fit.validate_fast(&core_options).into(),
            HValidInfoMode::Detailed => (&primary_fit.validate_verbose(&core_options)).into(),
        })
    }
}
