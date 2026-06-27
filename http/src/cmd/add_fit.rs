use serde::Deserialize;

use crate::{
    cmd::{HFitIdResp, basic::HFitAddCmdFCtxRIds},
    util::HExecError,
};

#[derive(Default, Deserialize)]
pub(crate) struct HFitAddCmd {
    #[serde(flatten)]
    basic: HFitAddCmdFCtxRIds,
}
impl HFitAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFitIdResp, HExecError> {
        self.basic.execute(core_sol)
    }
}
