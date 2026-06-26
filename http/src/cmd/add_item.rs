use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{HDroneAddCmdFCtxRIds, HImplantAddCmdFCtxRIds},
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemAddCmd {
    Drone(HDroneAddCmdFCtxRIds),
    Implant(HImplantAddCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
        }
    }
}
