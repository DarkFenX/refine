use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        basic_item::{HBoosterAddCmdFCtxRIds, HDroneAddCmdFCtxRIds, HImplantAddCmdFCtxRIds, HRigAddCmdFCtxRIds},
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemAddCmd {
    Booster(HBoosterAddCmdFCtxRIds),
    Drone(HDroneAddCmdFCtxRIds),
    Implant(HImplantAddCmdFCtxRIds),
    Rig(HRigAddCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemAddCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Booster(cmd) => cmd.execute(core_sol),
            Self::Drone(cmd) => cmd.execute(core_sol),
            Self::Implant(cmd) => cmd.execute(core_sol),
            Self::Rig(cmd) => cmd.execute(core_sol),
        }
    }
}
