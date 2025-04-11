use crate::{
    cmd::shared::HValOptions,
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
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fit_fast(fit_id, &core_options).map(|v| v.into()),
            HValidInfoMode::Detailed => core_sol
                .validate_fit_verbose(fit_id, &core_options)
                .map(|v| (&v).into()),
        }
        .map_err(|core_error| match core_error {
            rc::err::ValidateFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
        })
    }
}
