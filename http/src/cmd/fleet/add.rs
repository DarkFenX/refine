use serde::Deserialize;

use crate::{
    cmd::{basic::HFleetAddCmdFCtxRIds, shared::HCreatedFleetIdResp},
    err::HExecError,
};

#[derive(Default, Deserialize)]
pub(crate) struct HFleetAddCmd {
    #[serde(flatten)]
    basic: HFleetAddCmdFCtxRIds,
}
impl HFleetAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedFleetIdResp, HExecError> {
        self.basic.execute(core_sol)
    }
}
