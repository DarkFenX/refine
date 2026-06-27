use serde::Deserialize;

use crate::{
    cmd::{HFleetIdResp, basic::HFleetAddCmdFCtxRIds},
    util::HExecError,
};

#[derive(Default, Deserialize)]
pub(crate) struct HFleetAddCmd {
    #[serde(flatten)]
    basic: HFleetAddCmdFCtxRIds,
}
impl HFleetAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFleetIdResp, HExecError> {
        self.basic.execute(core_sol)
    }
}
