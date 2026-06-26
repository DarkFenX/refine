use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp, HCmdResps,
        basic_item::{
            HBoosterAddCmdFCtxBIds, HBoosterAddCmdFCtxRIds, HBoosterChangeCmdFCtxBIds, HBoosterChangeCmdFCtxRIds,
            HDroneAddCmdFCtxBIds, HDroneAddCmdFCtxRIds, HDroneChangeCmdFCtxBIds, HDroneChangeCmdFCtxRIds,
            HImplantAddCmdFCtxBIds, HImplantAddCmdFCtxRIds, HImplantChangeCmdFCtxBIds, HImplantChangeCmdFCtxRIds,
            HRigAddCmdFCtxBIds, HRigAddCmdFCtxRIds, HRigChangeCmdFCtxBIds, HRigChangeCmdFCtxRIds,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSolChangeCmdBIds {
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxBIds),
    ChangeBooster(HBoosterChangeCmdFCtxBIds),
    // Item - drone
    AddDrone(HDroneAddCmdFCtxBIds),
    ChangeDrone(HDroneChangeCmdFCtxBIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxBIds),
    ChangeImplant(HImplantChangeCmdFCtxBIds),
    // Item - rig
    AddRig(HRigAddCmdFCtxBIds),
    ChangeRig(HRigChangeCmdFCtxBIds),
}

pub(crate) enum HSolChangeCmdRIds {
    // Item - booster
    AddBooster(HBoosterAddCmdFCtxRIds),
    ChangeBooster(HBoosterChangeCmdFCtxRIds),
    // Item - drone
    AddDrone(HDroneAddCmdFCtxRIds),
    ChangeDrone(HDroneChangeCmdFCtxRIds),
    // Item - implant
    AddImplant(HImplantAddCmdFCtxRIds),
    ChangeImplant(HImplantChangeCmdFCtxRIds),
    // Item - rig
    AddRig(HRigAddCmdFCtxRIds),
    ChangeRig(HRigChangeCmdFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdBIds {
    pub(crate) fn render(self, resps: &HCmdResps) -> Result<HSolChangeCmdRIds, HExecError> {
        Ok(match self {
            // Item - booster
            Self::AddBooster(cmd) => HSolChangeCmdRIds::AddBooster(cmd.render(resps)?),
            Self::ChangeBooster(cmd) => HSolChangeCmdRIds::ChangeBooster(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => HSolChangeCmdRIds::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => HSolChangeCmdRIds::ChangeDrone(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => HSolChangeCmdRIds::AddImplant(cmd.render(resps)?),
            Self::ChangeImplant(cmd) => HSolChangeCmdRIds::ChangeImplant(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => HSolChangeCmdRIds::AddRig(cmd.render(resps)?),
            Self::ChangeRig(cmd) => HSolChangeCmdRIds::ChangeRig(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolChangeCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
