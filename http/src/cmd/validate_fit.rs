use crate::{
    info::{HValidInfo, HValidInfoMode},
    util::HExecError,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HValidFitCmd {
    Include(Vec<String>),
    Exclude(Vec<String>),
}
impl HValidFitCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
        valid_mode: HValidInfoMode,
    ) -> Result<HValidInfo, HExecError> {
        // Extract options
        let (mut core_options, options, alt_value) = match self {
            Self::Include(options) => (rc::SolValOptions::new_disabled(), options, true),
            Self::Exclude(options) => (rc::SolValOptions::new_enabled(), options, false),
        };
        for option in options {
            match option.to_lowercase().as_str() {
                "cpu" => core_options.cpu = alt_value,
                "pg" => core_options.pg = alt_value,
                _ => (),
            }
        }
        // Run validation
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fit_fast(fit_id, core_options).map(|v| v.into()),
            HValidInfoMode::Detailed => core_sol.validate_fit_verbose(fit_id, core_options).map(|v| (&v).into()),
        }
        .map_err(|core_error| match core_error {
            rc::err::ValidateFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
        })
    }
}
impl Default for HValidFitCmd {
    fn default() -> Self {
        Self::Exclude(Vec::new())
    }
}
