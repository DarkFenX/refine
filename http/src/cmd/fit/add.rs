use serde::Deserialize;

use crate::{
    cmd::{basic::HFitAddCmdFCtxRIds, shared::HCreatedFitIdResp},
    util::HExecError,
};

#[derive(Default, Deserialize)]
pub(crate) struct HFitAddCmd {
    #[serde(flatten)]
    basic: HFitAddCmdFCtxRIds,
}
impl HFitAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedFitIdResp, HExecError> {
        self.basic.execute(core_sol)
    }
}
