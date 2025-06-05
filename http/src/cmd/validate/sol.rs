use crate::{
    cmd::shared::HValOptions,
    info::{HSolValResult, HValidInfoMode},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize, Default)]
pub(crate) struct HValidateSolCmd {
    #[serde(default)]
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    fit_ids: Vec<rc::FitId>,
    options: HValOptions,
}
impl HValidateSolCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, valid_mode: HValidInfoMode) -> HSolValResult {
        let core_options = rc::val::ValOptionsSol {
            fit_ids: self.fit_ids.clone(),
            options: self.options.to_core(),
        };
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fast(&core_options).into(),
            HValidInfoMode::Detailed => (&core_sol.validate_verbose(&core_options)).into(),
        }
    }
}
