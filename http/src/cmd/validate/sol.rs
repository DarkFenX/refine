use crate::{
    cmd::shared::HValOptionsSol,
    info::{HSolValResult, HValidInfoMode},
};

#[derive(serde::Deserialize, Default)]
#[serde(transparent)]
pub(crate) struct HValidateSolCmd {
    validation_options: HValOptionsSol,
}
impl HValidateSolCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, valid_mode: HValidInfoMode) -> HSolValResult {
        let core_options = self.validation_options.to_core();
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fast(&core_options).into(),
            HValidInfoMode::Detailed => (&core_sol.validate_verbose(&core_options)).into(),
        }
    }
}
