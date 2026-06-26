use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HDroneAddCmdFCtxBIds, HDroneAddCmdFCtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds,
            HImplantAddCmdFCtxBIds, HImplantAddCmdFCtxRIds, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSolChangeCmdBIds {
    // Item - drone
    AddDrone(HDroneAddCmdFCtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxBIds),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
}

pub(crate) enum HSolChangeCmdRIds {
    // Item - drone
    AddDrone(HDroneAddCmdFCtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxRIds),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdBIds {
    pub(crate) fn render(self, resps: &HCmdResps) -> Result<HSolChangeCmdRIds, HExecError> {
        Ok(match self {
            // Item - drone
            Self::AddDrone(cmd) => HSolChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HSolChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HSolChangeCmdRIds::AddImplant(cmd.render(resps)?),
            Self::ChangeImplant(cmd) => HSolChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
