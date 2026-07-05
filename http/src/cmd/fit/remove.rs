use serde::Deserialize;

use crate::{cmd::basic::HFitRemoveCmdICtx, err::HExecError};

#[derive(Default, Deserialize)]
pub(crate) struct HFitRemoveCmd {
    #[serde(flatten)]
    basic: HFitRemoveCmdICtx,
}
impl HFitRemoveCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        self.basic.execute(core_sol, fit_id)
    }
}
