use serde::Deserialize;

use crate::{
    cmd::{HCmdResp, old_change_fit::HChangeFitCmd},
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeFitCommand {
    // Fit
    ChangeFit(HChangeFitCmd),
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
        }
    }
}
